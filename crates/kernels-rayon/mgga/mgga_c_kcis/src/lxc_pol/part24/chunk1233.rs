//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1233/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1233(t1092: f64, t28991: f64, t92917: f64, t1662: f64, t28182: f64, t92693: f64, t20345: f64, t20349: f64, t20573: f64, t26955: f64, t26960: f64, t26966: f64, t28153: f64, t28204: f64, t29094: f64, t95581: f64, t95585: f64, t96781: f64, t97083: f64, t97089: f64, t97093: f64) -> (f64, f64) {
    let t100108 = t1092 * t92917 * t28991;
    let t100114 = t92693 * t1662 * t28182;
    let t100128 = t96781 - 0.41270617283950617283e-2_f64 * t95581 + 0.92754700520833333334e-4_f64 * t28204 * t28153 - 0.23214722222222222222e-2_f64 * t100108 - 0.46336805555555555556e-3_f64 * t26960 * t97093 * t20573 - 0.23168402777777777778e-3_f64 * t26960 * t100114 - 0.46336805555555555556e-3_f64 * t26960 * t97083 * t20345 - 0.30918233506944444445e-4_f64 * t26955 * t100114 + 0.30891203703703703704e-3_f64 * t26960 * t97089 * t20349 + 0.18534722222222222222e-2_f64 * t26966 * t29094 + 0.46429444444444444444e-2_f64 * t95585;
    (t100108, t100128)
}
