//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk883;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk884;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk885;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk886;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta173(t1294: f64, t3737: f64, t1204: f64, t1284: f64, t1280: f64, t3568: f64, t487: f64, t1209: f64, t1287: f64, t3721: f64, t1269: f64, t473: f64, t1214: f64, t3584: f64, t3140: f64, t3596: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3738 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk883(t1294);
        let t3739 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk884(t3737, t3738);
        let t3746 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk885(t1204, t1284);
        let (t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk886(t1280, t3568, t1284, t487, t1209, t1287, t3721, t1269, t473, t1214, t3584, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk887(t3766, t460);
    (t3738, t3739, t3746, t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766, t3767)
}
