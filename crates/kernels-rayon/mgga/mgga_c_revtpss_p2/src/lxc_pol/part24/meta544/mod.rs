//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1606;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1607;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1608;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1609;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1610;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1611;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta544(t76959: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t87303: f64, t87304: f64, t87305: f64, t87306: f64, t87307: f64, t87309: f64, t87312: f64, t87314: f64, t76965: f64, t6071: f64, t18800: f64, t23384: f64, t23414: f64, t2770: f64, t39549: f64, t4474: f64, t50155: f64, t50166: f64, t50178: f64, t6049: f64, t61324: f64, t61330: f64, t61337: f64, t61355: f64, t75950: f64, t75956: f64, t75961: f64, t865: f64, t39554: f64, t39557: f64, t50205: f64, t50214: f64, t61361: f64, t61367: f64, t61371: f64, t61397: f64, t61400: f64, t61407: f64, t61411: f64, t75974: f64, t75978: f64, t75984: f64, t75998: f64, t76010: f64, t6048: f64, t40998: f64, t41003: f64, t41037: f64, t41049: f64, t41078: f64, t50248: f64, t51203: f64, t51211: f64, t61448: f64, t62528: f64, t76020: f64, t76026: f64, t76051: f64, t76058: f64, t76062: f64, t6016: f64, t2723: f64, t5977: f64, t231: f64, t5966: f64, t10770: f64, t10871: f64, t14586: f64, t14791: f64, t14894: f64, t1544: f64, t1559: f64, t18426: f64, t18444: f64, t18469: f64, t18627: f64, t23245: f64, t2745: f64, t2747: f64, t40673: f64, t4362: f64, t4364: f64, t4365: f64, t5962: f64, t6017: f64, t6022: f64, t6035: f64, t76284: f64, t76289: f64, t76313: f64, t76315: f64, t76330: f64, t76337: f64, t76362: f64, t76705: f64, t23160: f64, t23334: f64, t50370: f64, t50372: f64, t50377: f64, t50381: f64, t50385: f64, t61570: f64, t61572: f64, t61576: f64, t61623: f64, t61645: f64, t61675: f64, t76321: f64, t76428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87315, t87316) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1606(t76959, t39483, t39520, t39528, t39531, t39534, t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314);
        let (t87318, t87342) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1607(t76965, t6071, t18800, t23384, t23414, t2770, t39549, t4474, t50155, t50166, t50178, t6049, t61324, t61330, t61337, t61355, t75950, t75956, t75961, t865);
        let t87357 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1608(t39554, t39557, t50205, t50214, t61361, t61367, t61371, t61397, t61400, t61407, t61411, t75974, t75978, t75984, t75998, t76010);
        let t87373 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1609(t6048, t40998, t41003, t41037, t41049, t41078, t50248, t51203, t51211, t61448, t62528, t76020, t76026, t76051, t76058, t76062, t865);
        let (t87394, t87395, t87399, t87400, t87470) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1610(t6016, t2723, t5977, t231, t5966, t10770, t10871, t14586, t14791, t14894, t1544, t1559, t18426, t18444, t18469, t18627, t23245, t2745, t2747, t40673, t4362, t4364, t4365, t5962, t6017, t6022, t6035, t76284, t76289, t76313, t76315, t76330, t76337, t76362, t76705);
        let t87503 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1611(t14791, t18426, t18627, t23160, t23334, t2745, t2747, t4362, t4364, t50370, t50372, t50377, t50381, t50385, t6017, t6035, t61570, t61572, t61576, t61623, t61645, t61675, t76284, t76289, t76321, t76428);
    (t87315, t87316, t87318, t87342, t87357, t87373, t87394, t87395, t87399, t87400, t87470, t87503)
}
