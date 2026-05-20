//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1882;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1883;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta412<F: Float>(t1275: F, t225: F, t1294: F, t3738: F, t1204: F, t1210: F, t1215: F, t12666: F, t12673: F, t12690: F, t12696: F, t1271: F, t1274: F, t1295: F, t13166: F, t13170: F, t13174: F, t13177: F, t3552: F, t3556: F, t3561: F, t3585: F, t3729: F, t3732: F, t3739: F, t3791: F, t460: F, t495: F, t12663: F, t12413: F, t12417: F, t12566: F, t12573: F, t12575: F, t12577: F, t12579: F, t12583: F, t12584: F, t12587: F, t12594: F, t12598: F, t1298: F, t1300: F, t198: F, t336: F, t3794: F, t3801: F, t5023: F, t33: F, t265: F, t502: F, t11095: F, t12562: F, t10326: F, t1113: F, t1304: F, t2258: F, t2838: F, t3351: F, t3805: F, t504: F, t57: F, t606: F, t895: F, t9357: F, dens_threshold: F, rho1: F, zeta_threshold: F, t12211: F, t1310: F, t2371: F, t10192: F, t10194: F, t10260: F, t10263: F, t10415: F, t10416: F, t10426: F, t118: F, t1315: F, t1453: F, t2320: F, t2322: F, t2328: F, t2331: F, t2372: F, t3813: F, t3821: F, t4151: F, t4254: F, t508: F, t511: F, t569: F, t649: F, t651: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13180, t13181, t13182, t13183, t13184, t13189) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1881::<F>(t1275, t225, t1294, t3738, t1204, t1210, t1215, t12666, t12673, t12690, t12696, t1271, t1274, t1295, t13166, t13170, t13174, t13177, t3552, t3556, t3561, t3585, t3729, t3732, t3739, t3791, t460, t495);
        let (t13190, t13194) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1882::<F>(t12663, t13189, t12413, t12417, t12566, t12573, t12575, t12577, t12579, t12583, t12584, t12587, t12594, t12598, t1298, t1300, t198, t336, t3794, t3801, t5023);
        let (t13196, t13206) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1883::<F>(t33, t265, t502, t11095, t12562, t13194, t10326, t1113, t1304, t2258, t2838, t3351, t3805, t504, t57, t606, t895, t9357, dens_threshold, rho1, zeta_threshold);
        let (t13207, t13216, t13225) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1884::<F>(t12211, t13206, t1310, t2371, t10192, t10194, t10260, t10263, t10415, t10416, t10426, t118, t1315, t1453, t2320, t2322, t2328, t2331, t2372, t3813, t3821, t4151, t4254, t508, t511, t569, t649, t651, t671);
    (t13180, t13181, t13182, t13183, t13184, t13190, t13196, t13207, t13216, t13225)
}
