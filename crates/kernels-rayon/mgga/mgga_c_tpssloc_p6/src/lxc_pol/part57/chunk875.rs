//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 875/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk875(t1799: f64, t2018: f64, t24432: f64, t22574: f64, t7685: f64, t8644: f64, t191: f64, t192: f64, t7900: f64, t2020: f64, t7754: f64, t8607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33357 = t2018 * t1799;
    let t33358 = t24432 * t33357;
    let t33360 = 3.0_f64 * t22574 * t33358;
    let t33361 = t7685 * t8644;
    let t33363 = t7900 * t191 * t192;
    let t33364 = t33363 * t2020;
    let t33365 = t8607 * t7754;
    (t33357, t33358, t33360, t33361, t33363, t33364, t33365)
}
