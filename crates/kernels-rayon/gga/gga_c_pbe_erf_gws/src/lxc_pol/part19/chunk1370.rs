//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1370/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1370(t1144: f64, t14886: f64, t4386: f64, t12255: f64, t14182: f64, t14188: f64, t14193: f64, t14881: f64, t15036: f64, t2408: f64, t29775: f64, t34922: f64, t35057: f64, t36323: f64, t39689: f64, t54957: f64, t54978: f64, t55065: f64, t55344: f64, t55345: f64, t56773: f64, t6793: f64, t8629: f64, t8793: f64, t9283: f64) -> f64 {
    let t58384 = t4386 * t1144 * t14886;
    let t58410 = t34922 * t14193 / 96.0_f64 + t6793 * t58384 / 24.0_f64 + t36323 * t14188 / 48.0_f64 + t36323 * t14182 / 48.0_f64 + t39689 * t14182 / 48.0_f64 + t8793 * t55065 / 24.0_f64 + t39689 * t14188 / 48.0_f64 + t29775 * t15036 / 24.0_f64 + t35057 * t14188 / 48.0_f64 + t8629 * t54978 / 48.0_f64 + t8793 * t54957 / 24.0_f64 + t2408 * t9283 * t14881 * t12255 / 8.0_f64 + t56773 / 48.0_f64 - t55344 + t55345;
    t58410
}
