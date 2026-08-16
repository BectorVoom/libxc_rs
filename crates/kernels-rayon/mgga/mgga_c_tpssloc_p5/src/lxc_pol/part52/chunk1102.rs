//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1102/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1102(t33: f64, t7973: f64, t2240: f64, t12571: f64, t7245: f64, t1419: f64, t55: f64, t22510: f64, t24498: f64, t3961: f64, t3966: f64, t607: f64, t7251: f64) -> (f64, f64, f64, f64) {
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    let t27356 = t1419 * t55;
    let t27363 = 20.0_f64 / 9.0_f64 * t27356 * t607 + 5.0_f64 / 18.0_f64 * t24498 * t3961 - 5.0_f64 / 6.0_f64 * t7251 * t3966 - t22510;
    (t27331, t27332, t27341, t27363)
}
