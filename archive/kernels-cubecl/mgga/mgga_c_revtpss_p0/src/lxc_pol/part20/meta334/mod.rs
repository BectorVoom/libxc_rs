//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1253;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1254;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1255;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta334<F: Float>(t12766: F, t13164: F, t1277: F, t13107: F, t225: F, t494: F, t1214: F, t3738: F, t3737: F, t1269: F, t3555: F, t1275: F, t1294: F, t1204: F, t1210: F, t1215: F, t12666: F, t12673: F, t12690: F, t12696: F, t1271: F, t1274: F, t1295: F, t3552: F, t3556: F, t3561: F, t3585: F, t3729: F, t3732: F, t3739: F, t3791: F, t460: F, t495: F, t12663: F, t12413: F, t12417: F, t12566: F, t12573: F, t12575: F, t12577: F, t12579: F, t12583: F, t12584: F, t12587: F, t12594: F, t12598: F, t1298: F, t1300: F, t198: F, t336: F, t3794: F, t3801: F, t5023: F, t33: F, t265: F, t502: F, t11095: F, t12562: F, t10326: F, t1113: F, t1304: F, t2258: F, t2838: F, t3351: F, t3805: F, t504: F, t57: F, t606: F, t895: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13165, t13166, t13170, t13174, t13177, t13180) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1253::<F>(t12766, t13164, t1277, t13107, t225, t494, t1214, t3738, t3737, t1269, t3555, t1275);
        let (t13182, t13183, t13184, t13189) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1254::<F>(t13180, t225, t1294, t3738, t1204, t1210, t1215, t12666, t12673, t12690, t12696, t1271, t1274, t1295, t13166, t13170, t13174, t13177, t3552, t3556, t3561, t3585, t3729, t3732, t3739, t3791, t460, t495);
        let (t13190, t13194) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1255::<F>(t12663, t13189, t12413, t12417, t12566, t12573, t12575, t12577, t12579, t12583, t12584, t12587, t12594, t12598, t1298, t1300, t198, t336, t3794, t3801, t5023);
        let (t13196, t13206) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1256::<F>(t33, t265, t502, t11095, t12562, t13194, t10326, t1113, t1304, t2258, t2838, t3351, t3805, t504, t57, t606, t895, t9357, dens_threshold, rho1, zeta_threshold);
    (t13165, t13166, t13170, t13174, t13177, t13180, t13182, t13183, t13184, t13190, t13196, t13206)
}
