//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1971;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1972;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1973;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta542<F: Float>(t1215: F, t26922: F, t26949: F, t26994: F, t29264: F, t29268: F, t29272: F, t29275: F, t29279: F, t29283: F, t29287: F, t29293: F, t29297: F, t29301: F, t29304: F, t29308: F, t5237: F, t5429: F, t5498: F, t7602: F, t7632: F, t7636: F, t7639: F, t7643: F, t7651: F, t265: F, t502: F, t29154: F, t29210: F, t29258: F, t3801: F, t8220: F, t1298: F, t1832: F, t1300: F, t198: F, t27037: F, t27041: F, t27754: F, t336: F, t5023: F, t5501: F, t7673: F, t33: F, t1469: F, t2159: F, t27821: F, t4186: F, t57: F, t606: F, t7677: F, t8227: F, t29005: F, t118: F, t1502: F, t2163: F, t27116: F, t27118: F, t27120: F, t27122: F, t27125: F, t27128: F, t27130: F, t27132: F, t27134: F, t4246: F, t4293: F, t4297: F, t7586: F, t7683: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1479: F, t60: F, t25137: F, t26776: F, t4181: F, t7571: F, t72: F, t1927: F, t6977: F, t8143: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t29311 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1971::<F>(t1215, t26922, t26949, t26994, t29264, t29268, t29272, t29275, t29279, t29283, t29287, t29293, t29297, t29301, t29304, t29308, t5237, t5429, t5498, t7602, t7632, t7636, t7639, t7643, t7651);
        let (t29313, t29317, t29322, t29329) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1972::<F>(t265, t502, t29154, t29210, t29258, t29311, t3801, t8220, t1298, t1832, t1300, t198, t27037, t27041, t27754, t336, t5023, t5501, t7673);
        let (t29337, t29343) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1973::<F>(t33, t1469, t2159, t27821, t29329, t4186, t57, t606, t7677, t8227, t29005, t118, t1502, t2163, t27116, t27118, t27120, t27122, t27125, t27128, t27130, t27132, t27134, t4246, t4293, t4297, t7586, t7683, dens_threshold, rho1, zeta_threshold);
        let (t29355, t29362, t29363, t29364, t29367) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1974::<F>(t1479, t60, t25137, t26776, t4181, t4186, t606, t7571, t72, t1927, t6977, t8143);
    (t29313, t29317, t29322, t29329, t29337, t29343, t29355, t29362, t29363, t29364, t29367)
}
