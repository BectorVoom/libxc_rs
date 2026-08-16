//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1606;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1607;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1608;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1609;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1610;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta544<F: Float>(t76959: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t87303: F, t87304: F, t87305: F, t87306: F, t87307: F, t87309: F, t87312: F, t87314: F, t76965: F, t6071: F, t18800: F, t23384: F, t23414: F, t2770: F, t39549: F, t4474: F, t50155: F, t50166: F, t50178: F, t6049: F, t61324: F, t61330: F, t61337: F, t61355: F, t75950: F, t75956: F, t75961: F, t865: F, t39554: F, t39557: F, t50205: F, t50214: F, t61361: F, t61367: F, t61371: F, t61397: F, t61400: F, t61407: F, t61411: F, t75974: F, t75978: F, t75984: F, t75998: F, t76010: F, t6048: F, t40998: F, t41003: F, t41037: F, t41049: F, t41078: F, t50248: F, t51203: F, t51211: F, t61448: F, t62528: F, t76020: F, t76026: F, t76051: F, t76058: F, t76062: F, t6016: F, t2723: F, t5977: F, t231: F, t5966: F, t10770: F, t10871: F, t14586: F, t14791: F, t14894: F, t1544: F, t1559: F, t18426: F, t18444: F, t18469: F, t18627: F, t23245: F, t2745: F, t2747: F, t40673: F, t4362: F, t4364: F, t4365: F, t5962: F, t6017: F, t6022: F, t6035: F, t76284: F, t76289: F, t76313: F, t76315: F, t76330: F, t76337: F, t76362: F, t76705: F, t23160: F, t23334: F, t50370: F, t50372: F, t50377: F, t50381: F, t50385: F, t61570: F, t61572: F, t61576: F, t61623: F, t61645: F, t61675: F, t76321: F, t76428: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87315, t87316) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1606::<F>(t76959, t39483, t39520, t39528, t39531, t39534, t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314);
        let (t87318, t87342) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1607::<F>(t76965, t6071, t18800, t23384, t23414, t2770, t39549, t4474, t50155, t50166, t50178, t6049, t61324, t61330, t61337, t61355, t75950, t75956, t75961, t865);
        let t87357 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1608::<F>(t39554, t39557, t50205, t50214, t61361, t61367, t61371, t61397, t61400, t61407, t61411, t75974, t75978, t75984, t75998, t76010);
        let t87373 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1609::<F>(t6048, t40998, t41003, t41037, t41049, t41078, t50248, t51203, t51211, t61448, t62528, t76020, t76026, t76051, t76058, t76062, t865);
        let (t87394, t87395, t87399, t87400, t87470) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1610::<F>(t6016, t2723, t5977, t231, t5966, t10770, t10871, t14586, t14791, t14894, t1544, t1559, t18426, t18444, t18469, t18627, t23245, t2745, t2747, t40673, t4362, t4364, t4365, t5962, t6017, t6022, t6035, t76284, t76289, t76313, t76315, t76330, t76337, t76362, t76705);
        let t87503 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1611::<F>(t14791, t18426, t18627, t23160, t23334, t2745, t2747, t4362, t4364, t50370, t50372, t50377, t50381, t50385, t6017, t6035, t61570, t61572, t61576, t61623, t61645, t61675, t76284, t76289, t76321, t76428);
    (t87315, t87316, t87318, t87342, t87357, t87373, t87394, t87395, t87399, t87400, t87470, t87503)
}
