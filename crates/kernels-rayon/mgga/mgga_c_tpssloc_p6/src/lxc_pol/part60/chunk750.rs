//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 750/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk750(t112: f64, t7945: f64, t2109: f64, t26012: f64, t33: f64, t7973: f64, t2240: f64, t12571: f64, t7245: f64, t1419: f64, t55: f64, t1240: f64, t1760: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27254 = t7945 * t112;
    let t27298 = t2109 * t26012;
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    let t27356 = t1419 * t55;
    let t27381 = t1240 * t1760;
    (t27254, t27298, t27331, t27332, t27341, t27356, t27381)
}
