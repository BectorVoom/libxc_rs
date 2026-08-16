//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1690;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1691;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1692;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1693;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta562<F: Float>(t23842: F, t23911: F, t1651: F, t23640: F, t11250: F, t11774: F, t11927: F, t15700: F, t15701: F, t15707: F, t16222: F, t19738: F, t19741: F, t23633: F, t23892: F, t23900: F, t23904: F, t23964: F, t3091: F, t3092: F, t3117: F, t43105: F, t6266: F, t78676: F, t78750: F, t78756: F, t78763: F, t78802: F, t79159: F, t6258: F, t6305: F, t23598: F, t15962: F, t5819: F, t11704: F, t1063: F, t11257: F, t11703: F, t11853: F, t11875: F, t15618: F, t19501: F, t19611: F, t19878: F, t23470: F, t23474: F, t23917: F, t23966: F, t24013: F, t247: F, t3116: F, t3162: F, t3182: F, t42410: F, t42690: F, t4837: F, t4899: F, t54570: F, t78805: F, t78855: F, t88112: F, t88128: F, t3094: F, t5825: F, t1668: F, t1045: F, t11660: F, t15926: F, t16081: F, t19450: F, t23630: F, t23936: F, t23994: F, t3115: F, t42215: F, t4834: F, t4892: F, t53326: F, t6273: F, t67551: F, t78863: F, t80358: F, t88120: F, t1469: F, t22671: F, t22688: F, t1042: F, t1066: F, t15716: F, t16208: F, t23481: F, t3127: F, t43253: F, t4801: F, t4806: F, t65581: F, t65596: F, t78496: F, t78910: F, t78915: F, t78986: F, t88091: F, t88646: F, t88750: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88773, t88794) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1690::<F>(t23842, t23911, t1651, t23640);
        let t88800 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1691::<F>(t11250, t11774, t11927, t15700, t15701, t15707, t16222, t19738, t19741, t23633, t23892, t23900, t23904, t23911, t23964, t3091, t3092, t3117, t43105, t6266, t78676, t78750, t78756, t78763, t78802, t79159, t88773, t88794);
        let (t88804, t88815, t88828, t88844, t88849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1692::<F>(t6258, t6305, t1651, t23598, t15962, t5819, t11704, t1063, t11257, t11703, t11853, t11875, t15618, t19501, t19611, t19878, t23470, t23474, t23911, t23917, t23966, t24013, t247, t3091, t3092, t3116, t3117, t3162, t3182, t42410, t42690, t4837, t4899, t54570, t78805, t78855, t88112, t88128, t88794);
        let (t88885, t88898) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1693::<F>(t3094, t5825, t1668, t23598, t1045, t1063, t11660, t11703, t15926, t16081, t19450, t19501, t19611, t19741, t23630, t23936, t23994, t247, t3091, t3092, t3115, t3117, t3182, t42215, t4834, t4892, t4899, t53326, t5819, t6273, t67551, t78863, t80358, t88120, t88844);
        let (t88901, t88916, t88925, t88944) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1694::<F>(t5819, t6258, t1469, t22671, t1651, t22688, t1042, t1063, t1066, t11703, t15716, t16081, t16208, t23481, t23911, t247, t3091, t3092, t3116, t3127, t43253, t4801, t4806, t4837, t65581, t65596, t78496, t78910, t78915, t78986, t88091, t88646, t88750);
    (t88794, t88800, t88804, t88815, t88828, t88849, t88885, t88898, t88901, t88916, t88925, t88944)
}
