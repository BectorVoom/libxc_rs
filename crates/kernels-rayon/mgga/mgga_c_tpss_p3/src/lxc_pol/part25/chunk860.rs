//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 860/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk860(t27: f64, t559: f64, t20: f64, t571: f64, t12: f64, t558: f64, t1965: f64, t1971: f64, t567: f64, t1970: f64, t3: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7662 = t559 * t27;
    let t7665 = 120.0_f64 * t20 * t571;
    let t7666 = t12 * t558;
    let t7668 = 120.0_f64 * t7666 * t27;
    let t7669 = t1965 * t571;
    let t7671 = t567 * t1971;
    let t7673 = t1970 * t3;
    let t7674 = 1.0_f64 / t7673;
    let t7676 = 336.0_f64 * t25 * t7674;
    (t7662, t7665, t7668, t7669, t7671, t7676)
}
