//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1690;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1691;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1692;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1693;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta562(t23842: f64, t23911: f64, t1651: f64, t23640: f64, t11250: f64, t11774: f64, t11927: f64, t15700: f64, t15701: f64, t15707: f64, t16222: f64, t19738: f64, t19741: f64, t23633: f64, t23892: f64, t23900: f64, t23904: f64, t23964: f64, t3091: f64, t3092: f64, t3117: f64, t43105: f64, t6266: f64, t78676: f64, t78750: f64, t78756: f64, t78763: f64, t78802: f64, t79159: f64, t6258: f64, t6305: f64, t23598: f64, t15962: f64, t5819: f64, t11704: f64, t1063: f64, t11257: f64, t11703: f64, t11853: f64, t11875: f64, t15618: f64, t19501: f64, t19611: f64, t19878: f64, t23470: f64, t23474: f64, t23917: f64, t23966: f64, t24013: f64, t247: f64, t3116: f64, t3162: f64, t3182: f64, t42410: f64, t42690: f64, t4837: f64, t4899: f64, t54570: f64, t78805: f64, t78855: f64, t88112: f64, t88128: f64, t3094: f64, t5825: f64, t1668: f64, t1045: f64, t11660: f64, t15926: f64, t16081: f64, t19450: f64, t23630: f64, t23936: f64, t23994: f64, t3115: f64, t42215: f64, t4834: f64, t4892: f64, t53326: f64, t6273: f64, t67551: f64, t78863: f64, t80358: f64, t88120: f64, t1469: f64, t22671: f64, t22688: f64, t1042: f64, t1066: f64, t15716: f64, t16208: f64, t23481: f64, t3127: f64, t43253: f64, t4801: f64, t4806: f64, t65581: f64, t65596: f64, t78496: f64, t78910: f64, t78915: f64, t78986: f64, t88091: f64, t88646: f64, t88750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88773, t88794) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1690(t23842, t23911, t1651, t23640);
        let t88800 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1691(t11250, t11774, t11927, t15700, t15701, t15707, t16222, t19738, t19741, t23633, t23892, t23900, t23904, t23911, t23964, t3091, t3092, t3117, t43105, t6266, t78676, t78750, t78756, t78763, t78802, t79159, t88773, t88794);
        let (t88804, t88815, t88828, t88844, t88849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1692(t6258, t6305, t1651, t23598, t15962, t5819, t11704, t1063, t11257, t11703, t11853, t11875, t15618, t19501, t19611, t19878, t23470, t23474, t23911, t23917, t23966, t24013, t247, t3091, t3092, t3116, t3117, t3162, t3182, t42410, t42690, t4837, t4899, t54570, t78805, t78855, t88112, t88128, t88794);
        let (t88885, t88898) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1693(t3094, t5825, t1668, t23598, t1045, t1063, t11660, t11703, t15926, t16081, t19450, t19501, t19611, t19741, t23630, t23936, t23994, t247, t3091, t3092, t3115, t3117, t3182, t42215, t4834, t4892, t4899, t53326, t5819, t6273, t67551, t78863, t80358, t88120, t88844);
        let (t88901, t88916, t88925, t88944) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1694(t5819, t6258, t1469, t22671, t1651, t22688, t1042, t1063, t1066, t11703, t15716, t16081, t16208, t23481, t23911, t247, t3091, t3092, t3116, t3127, t43253, t4801, t4806, t4837, t65581, t65596, t78496, t78910, t78915, t78986, t88091, t88646, t88750);
    (t88794, t88800, t88804, t88815, t88828, t88849, t88885, t88898, t88901, t88916, t88925, t88944)
}
