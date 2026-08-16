//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta970 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3238;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3239;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3240;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3241;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3242;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta970(t2439: f64, t2440: f64, t6049: f64, t14472: f64, t1580: f64, t2444: f64, t689: f64, t136: f64, t2457: f64, t41011: f64, t6048: f64, t10504: f64, t6071: f64, t11007: f64, t252: f64, t2782: f64, t886: f64, t14481: f64, t1569: f64, t2771: f64, t40970: f64, t40978: f64, t40986: f64, t40988: f64, t41078: f64, t50214: f64, t50218: f64, t50220: f64, t50222: f64, t50227: f64, t50232: f64, t50236: f64, t865: f64, t18805: f64, t41066: f64, t10995: f64, t122: f64, t18796: f64, t2466: f64, t11044: f64, t18797: f64, t18317: f64, t2435: f64, t10770: f64, t14791: f64, t14917: f64, t18426: f64, t2724: f64, t2745: f64, t40337: f64, t40357: f64, t40361: f64, t4362: f64, t4364: f64, t50292: f64, t50296: f64, t50298: f64, t50303: f64, t50308: f64, t51049: f64, t6035: f64, t45: f64, t13312: f64, t13396: f64, t1490: f64, t18281: f64, t18367: f64, t18372: f64, t2251: f64, t2258: f64, t4328: f64, t5860: f64, t5862: f64, t606: f64, t60717: f64, t60754: f64, t766: f64, t80: f64, zeta_threshold: f64, t57: f64, t1491: f64, t18379: f64, t18384: f64, t4335: f64, t5864: f64, t5866: f64, t770: f64, t83: f64, t10871: f64, t5977: f64, t14931: f64, t18477: f64, t51123: f64, t10811: f64, t18471: f64, t18451: f64, t124: f64, t14772: f64, t14786: f64, t14802: f64, t14894: f64, t1559: f64, t18632: f64, t50312: f64, t50325: f64, t50328: f64, t50347: f64, t51014: f64, t799: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61397, t61400, t61403, t61407, t61411) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3238(t2439, t2440, t6049, t14472, t1580, t2444, t689, t136, t2457, t41011, t6048, t10504, t6071);
        let t61429 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3239(t11007, t252, t2782, t6048, t886, t14481, t1569, t2771, t40970, t40978, t40986, t40988, t41078, t50214, t50218, t50220, t50222, t50227, t50232, t50236, t61397, t61400, t61403, t61407, t61411, t865);
        let (t61430, t61437, t61441, t61448, t61471) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3240(t18805, t41066, t10995, t122, t18796, t2466, t11044, t18797, t18317, t2435, t10770, t14791, t14917, t18426, t2724, t2745, t40337, t40357, t40361, t4362, t4364, t50292, t50296, t50298, t50303, t50308, t51049, t6035);
        let t61496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3241(t45, t13312, t13396, t1490, t18281, t18367, t18372, t2251, t2258, t4328, t5860, t5862, t606, t60717, t60754, t766, t80, zeta_threshold);
        let t61517 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3242(t57, t13312, t13396, t1491, t18281, t18379, t18384, t2251, t2258, t4335, t5864, t5866, t606, t60717, t60754, t770, t83, zeta_threshold);
        let (t61519, t61532, t61544) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3243(t61496, t61517, t10871, t5977, t14931, t18477, t51123, t10811, t18471, t18451, t124, t14772, t14786, t14791, t14802, t14894, t1559, t18632, t2745, t4362, t50312, t50325, t50328, t50347, t51014, t799, t800);
    (t61429, t61430, t61437, t61441, t61448, t61471, t61519, t61532, t61544)
}
