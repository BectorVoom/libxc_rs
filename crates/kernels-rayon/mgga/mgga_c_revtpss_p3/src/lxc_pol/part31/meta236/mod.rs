//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1058;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1059;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1060;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1061;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta236(t373: f64, t6244: f64, t371: f64, t372: f64, t1041: f64, t1063: f64, t1671: f64, t1675: f64, t3150: f64, t3161: f64, t3203: f64, t3205: f64, t375: f64, t4834: f64, t4846: f64, t4879: f64, t4925: f64, t6302: f64, t6308: f64, t6312: f64, t6318: f64, t6323: f64, t6327: f64, t6331: f64, t6298: f64, t225: f64, t385: f64, t1695: f64, t3269: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t6337, t6339) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1058(t373, t6244, t371, t372);
        let t6342 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1059(t1041, t1063, t1671, t1675, t3150, t3161, t3203, t3205, t375, t4834, t4846, t4879, t4925, t6302, t6308, t6312, t6318, t6323, t6327, t6331, t6339);
        let t6343 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1060(t6298, t6342);
        let (t6345, t6350) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1061(t225, t385, t6343, t1695);
        let t6351 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1062(t3269, t6350);
    (t6337, t6339, t6343, t6345, t6350, t6351)
}
