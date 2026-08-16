//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1253;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1254;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1255;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta334(t12766: f64, t13164: f64, t1277: f64, t13107: f64, t225: f64, t494: f64, t1214: f64, t3738: f64, t3737: f64, t1269: f64, t3555: f64, t1275: f64, t1294: f64, t1204: f64, t1210: f64, t1215: f64, t12666: f64, t12673: f64, t12690: f64, t12696: f64, t1271: f64, t1274: f64, t1295: f64, t3552: f64, t3556: f64, t3561: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t3791: f64, t460: f64, t495: f64, t12663: f64, t12413: f64, t12417: f64, t12566: f64, t12573: f64, t12575: f64, t12577: f64, t12579: f64, t12583: f64, t12584: f64, t12587: f64, t12594: f64, t12598: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t3794: f64, t3801: f64, t5023: f64, t33: f64, t265: f64, t502: f64, t11095: f64, t12562: f64, t10326: f64, t1113: f64, t1304: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t504: f64, t57: f64, t606: f64, t895: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13165, t13166, t13170, t13174, t13177, t13180) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1253(t12766, t13164, t1277, t13107, t225, t494, t1214, t3738, t3737, t1269, t3555, t1275);
        let (t13182, t13183, t13184, t13189) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1254(t13180, t225, t1294, t3738, t1204, t1210, t1215, t12666, t12673, t12690, t12696, t1271, t1274, t1295, t13166, t13170, t13174, t13177, t3552, t3556, t3561, t3585, t3729, t3732, t3739, t3791, t460, t495);
        let (t13190, t13194) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1255(t12663, t13189, t12413, t12417, t12566, t12573, t12575, t12577, t12579, t12583, t12584, t12587, t12594, t12598, t1298, t1300, t198, t336, t3794, t3801, t5023);
        let (t13196, t13206) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1256(t33, t265, t502, t11095, t12562, t13194, t10326, t1113, t1304, t2258, t2838, t3351, t3805, t504, t57, t606, t895, t9357, dens_threshold, rho1, zeta_threshold);
    (t13165, t13166, t13170, t13174, t13177, t13180, t13182, t13183, t13184, t13190, t13196, t13206)
}
