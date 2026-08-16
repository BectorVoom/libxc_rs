//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1400;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1401;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta373(t13126: f64, t460: f64, t3727: f64, t473: f64, t11239: f64, t3596: f64, t13038: f64, t1269: f64, t3555: f64, t1275: f64, t225: f64, t10270: f64, t10272: f64, t10279: f64, t10281: f64, t10288: f64, t10290: f64, t10275: f64, t10278: f64, t10284: f64, t10287: f64, t10295: f64, t4171: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13127, t13133, t13141, t13142, t13147, t13148, t13177, t13180, t13181, t13182, t13261) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1400(t13126, t460, t3727, t473, t11239, t3596, t13038, t1269, t3555, t1275, t225, t10270);
        let (t13267, t13269) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1401(t10272, t10279, t10281, t10288, t10290, t10275, t10278, t10284, t10287, t10295, t13261, t4171, t602);
    (t13127, t13133, t13141, t13142, t13147, t13148, t13177, t13180, t13181, t13182, t13267, t13269)
}
