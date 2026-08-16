//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 806/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk806(t13105: f64, t381: f64, t1795: f64, t3225: f64, t3436: f64, t5025: f64, t10513: f64, t284: f64, t41: f64, t9545: f64, t9588: f64, t1094: f64, t5163: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14721 = t13105 * t381;
    let t14781 = t1795 * t3225;
    let t14785 = t5025 * t3436;
    let t14832 = t10513 * t284;
    let t14838 = t41 * t9545;
    let t14849 = t9588 * t3436;
    let t14874 = t5163 * t1094;
    (t14721, t14781, t14785, t14832, t14838, t14849, t14874)
}
