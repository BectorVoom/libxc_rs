//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1375/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1375(t14881: f64, t353: f64, t3721: f64, t859: f64, t1206: f64, t14182: f64, t14193: f64, t3028: f64, t3037: f64, t34773: f64, t34850: f64, t35057: f64, t35260: f64, t4083: f64, t54984: f64, t55695: f64, t55702: f64, t55717: f64, t55726: f64, t57358: f64, t57361: f64, t57371: f64, t57375: f64, t57379: f64, t6793: f64, t8629: f64, t8793: f64) -> f64 {
    let t58540 = t859 * t353 * t14881 * t3721;
    let t58547 = -7.0_f64 / 72.0_f64 * t57358 + t57361 / 384.0_f64 - t55695 - t55702 - t34773 * t859 * t353 * t1206 * t3037 / 48.0_f64 - 7.0_f64 / 1152.0_f64 * t57371 + t57375 / 24.0_f64 + t55726 + t57379 / 8.0_f64 + t34850 * t14193 / 96.0_f64 + t8793 * t55717 / 24.0_f64 + t8793 * t54984 / 24.0_f64 + t8629 * t859 * t353 * t1206 * t3028 / 96.0_f64 - t6793 * t58540 / 16.0_f64 + t35057 * t14182 / 48.0_f64 - t35260 * t4083 / 96.0_f64;
    t58547
}
