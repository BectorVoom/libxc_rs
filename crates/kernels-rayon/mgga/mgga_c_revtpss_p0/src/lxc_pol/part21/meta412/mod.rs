//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1882;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1883;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta412(t1275: f64, t225: f64, t1294: f64, t3738: f64, t1204: f64, t1210: f64, t1215: f64, t12666: f64, t12673: f64, t12690: f64, t12696: f64, t1271: f64, t1274: f64, t1295: f64, t13166: f64, t13170: f64, t13174: f64, t13177: f64, t3552: f64, t3556: f64, t3561: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t3791: f64, t460: f64, t495: f64, t12663: f64, t12413: f64, t12417: f64, t12566: f64, t12573: f64, t12575: f64, t12577: f64, t12579: f64, t12583: f64, t12584: f64, t12587: f64, t12594: f64, t12598: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t3794: f64, t3801: f64, t5023: f64, t33: f64, t265: f64, t502: f64, t11095: f64, t12562: f64, t10326: f64, t1113: f64, t1304: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t504: f64, t57: f64, t606: f64, t895: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t12211: f64, t1310: f64, t2371: f64, t10192: f64, t10194: f64, t10260: f64, t10263: f64, t10415: f64, t10416: f64, t10426: f64, t118: f64, t1315: f64, t1453: f64, t2320: f64, t2322: f64, t2328: f64, t2331: f64, t2372: f64, t3813: f64, t3821: f64, t4151: f64, t4254: f64, t508: f64, t511: f64, t569: f64, t649: f64, t651: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13180, t13181, t13182, t13183, t13184, t13189) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1881(t1275, t225, t1294, t3738, t1204, t1210, t1215, t12666, t12673, t12690, t12696, t1271, t1274, t1295, t13166, t13170, t13174, t13177, t3552, t3556, t3561, t3585, t3729, t3732, t3739, t3791, t460, t495);
        let (t13190, t13194) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1882(t12663, t13189, t12413, t12417, t12566, t12573, t12575, t12577, t12579, t12583, t12584, t12587, t12594, t12598, t1298, t1300, t198, t336, t3794, t3801, t5023);
        let (t13196, t13206) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1883(t33, t265, t502, t11095, t12562, t13194, t10326, t1113, t1304, t2258, t2838, t3351, t3805, t504, t57, t606, t895, t9357, dens_threshold, rho1, zeta_threshold);
        let (t13207, t13216, t13225) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1884(t12211, t13206, t1310, t2371, t10192, t10194, t10260, t10263, t10415, t10416, t10426, t118, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t4151, t4254, t508, t511, t569, t649, t651, t671);
    (t13180, t13181, t13182, t13183, t13184, t13190, t13196, t13207, t13216, t13225)
}
