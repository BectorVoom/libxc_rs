//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 855/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk855(t1103: f64, t1104: f64, t18443: f64, t347: f64, t6320: f64, t934: f64, t14117: f64, t313: f64, t1045: f64, t4600: f64, t4601: f64, t4625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18736 = t1103 * t1104 * t18443;
    let t18739 = t347 * t6320;
    let t18740 = t18739 * t934;
    let t18741 = t14117 * t18740;
    let t18744 = t313 * t6320;
    let t18745 = t18744 * t1045;
    let t18746 = t4600 * t18745;
    let t18749 = t4601 * t4625;
    (t18736, t18740, t18741, t18745, t18746, t18749)
}
