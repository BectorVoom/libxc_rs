//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1920;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1921;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta545(t265: f64, t502: f64, t29154: f64, t29210: f64, t29258: f64, t29311: f64, t3801: f64, t8220: f64, t1298: f64, t1832: f64, t1300: f64, t198: f64, t27037: f64, t27041: f64, t27754: f64, t336: f64, t5023: f64, t5501: f64, t7673: f64, t33: f64, t1469: f64, t2159: f64, t27821: f64, t4186: f64, t57: f64, t606: f64, t7677: f64, t8227: f64, t29005: f64, t118: f64, t1502: f64, t2163: f64, t27116: f64, t27118: f64, t27120: f64, t27122: f64, t27125: f64, t27128: f64, t27130: f64, t27132: f64, t27134: f64, t4246: f64, t4293: f64, t4297: f64, t7586: f64, t7683: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t1479: f64, t60: f64, t25137: f64, t26776: f64, t4181: f64, t7571: f64, t72: f64, t1927: f64, t6977: f64, t8143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29313, t29317, t29322, t29329) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1920(t265, t502, t29154, t29210, t29258, t29311, t3801, t8220, t1298, t1832, t1300, t198, t27037, t27041, t27754, t336, t5023, t5501, t7673);
        let (t29337, t29343) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1921(t33, t1469, t2159, t27821, t29329, t4186, t57, t606, t7677, t8227, t29005, t118, t1502, t2163, t27116, t27118, t27120, t27122, t27125, t27128, t27130, t27132, t27134, t4246, t4293, t4297, t7586, t7683, dens_threshold, rho1, zeta_threshold);
        let (t29355, t29362, t29363, t29364, t29367) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1922(t1479, t60, t25137, t26776, t4181, t4186, t606, t7571, t72, t1927, t6977, t8143);
    (t29313, t29317, t29322, t29329, t29337, t29343, t29355, t29362, t29363, t29364, t29367)
}
