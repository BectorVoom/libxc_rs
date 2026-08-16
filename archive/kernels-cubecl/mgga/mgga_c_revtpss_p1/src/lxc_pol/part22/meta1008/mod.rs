//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1008 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3448;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3449;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3450;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1008<F: Float>(t19380: F, t999: F, t3075: F, t6258: F, t4946: F, t15654: F, t1678: F, t19748: F, t4866: F, t20089: F, t3153: F, t11249: F, t6271: F, t42871: F, t6305: F, t1024: F, t12097: F, t12122: F, t12127: F, t12149: F, t15655: F, t15886: F, t16152: F, t16450: F, t16458: F, t16552: F, t16554: F, t16561: F, t1692: F, t19414: F, t19488: F, t19556: F, t3151: F, t3204: F, t3278: F, t3291: F, t43520: F, t43524: F, t4857: F, t4970: F, t4976: F, t4983: F, t4998: F, t5004: F, t55499: F, t55887: F, t55938: F, t55939: F, t6379: F, t73: F, t225: F, t64816: F, t15648: F, t1651: F, t3133: F, t6244: F, t42078: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t64831, t64835, t64841, t64845, t64848, t64854, t64861) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3448::<F>(t19380, t999, t3075, t6258, t4946, t15654, t1678, t19748, t4866, t20089, t3153, t11249, t6271);
        let t64891 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3449::<F>(t42871, t6305);
        let t64896 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3450::<F>(t1024, t12097, t12122, t12127, t12149, t15655, t15886, t16152, t16450, t16458, t16552, t16554, t16561, t1692, t19414, t19488, t19556, t20089, t3075, t3151, t3204, t3278, t3291, t43520, t43524, t4857, t4970, t4976, t4983, t4998, t5004, t55499, t55887, t55938, t55939, t6379, t64848, t64854, t64861, t64891, t73);
        let (t64907, t64912, t64916, t64945) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3451::<F>(t225, t64816, t15648, t1651, t3133, t6244, t42078, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
    (t64831, t64835, t64841, t64845, t64848, t64861, t64891, t64896, t64907, t64912, t64916, t64945)
}
