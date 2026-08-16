//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1008 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3448;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3449;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3450;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1008(t19380: f64, t999: f64, t3075: f64, t6258: f64, t4946: f64, t15654: f64, t1678: f64, t19748: f64, t4866: f64, t20089: f64, t3153: f64, t11249: f64, t6271: f64, t42871: f64, t6305: f64, t1024: f64, t12097: f64, t12122: f64, t12127: f64, t12149: f64, t15655: f64, t15886: f64, t16152: f64, t16450: f64, t16458: f64, t16552: f64, t16554: f64, t16561: f64, t1692: f64, t19414: f64, t19488: f64, t19556: f64, t3151: f64, t3204: f64, t3278: f64, t3291: f64, t43520: f64, t43524: f64, t4857: f64, t4970: f64, t4976: f64, t4983: f64, t4998: f64, t5004: f64, t55499: f64, t55887: f64, t55938: f64, t55939: f64, t6379: f64, t73: f64, t225: f64, t64816: f64, t15648: f64, t1651: f64, t3133: f64, t6244: f64, t42078: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64831, t64835, t64841, t64845, t64848, t64854, t64861) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3448(t19380, t999, t3075, t6258, t4946, t15654, t1678, t19748, t4866, t20089, t3153, t11249, t6271);
        let t64891 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3449(t42871, t6305);
        let t64896 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3450(t1024, t12097, t12122, t12127, t12149, t15655, t15886, t16152, t16450, t16458, t16552, t16554, t16561, t1692, t19414, t19488, t19556, t20089, t3075, t3151, t3204, t3278, t3291, t43520, t43524, t4857, t4970, t4976, t4983, t4998, t5004, t55499, t55887, t55938, t55939, t6379, t64848, t64854, t64861, t64891, t73);
        let (t64907, t64912, t64916, t64945) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3451(t225, t64816, t15648, t1651, t3133, t6244, t42078, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
    (t64831, t64835, t64841, t64845, t64848, t64861, t64891, t64896, t64907, t64912, t64916, t64945)
}
