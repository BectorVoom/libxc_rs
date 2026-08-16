//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta970 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3238;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3239;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3240;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3241;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3242;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta970<F: Float>(t2439: F, t2440: F, t6049: F, t14472: F, t1580: F, t2444: F, t689: F, t136: F, t2457: F, t41011: F, t6048: F, t10504: F, t6071: F, t11007: F, t252: F, t2782: F, t886: F, t14481: F, t1569: F, t2771: F, t40970: F, t40978: F, t40986: F, t40988: F, t41078: F, t50214: F, t50218: F, t50220: F, t50222: F, t50227: F, t50232: F, t50236: F, t865: F, t18805: F, t41066: F, t10995: F, t122: F, t18796: F, t2466: F, t11044: F, t18797: F, t18317: F, t2435: F, t10770: F, t14791: F, t14917: F, t18426: F, t2724: F, t2745: F, t40337: F, t40357: F, t40361: F, t4362: F, t4364: F, t50292: F, t50296: F, t50298: F, t50303: F, t50308: F, t51049: F, t6035: F, t45: F, t13312: F, t13396: F, t1490: F, t18281: F, t18367: F, t18372: F, t2251: F, t2258: F, t4328: F, t5860: F, t5862: F, t606: F, t60717: F, t60754: F, t766: F, t80: F, zeta_threshold: F, t57: F, t1491: F, t18379: F, t18384: F, t4335: F, t5864: F, t5866: F, t770: F, t83: F, t10871: F, t5977: F, t14931: F, t18477: F, t51123: F, t10811: F, t18471: F, t18451: F, t124: F, t14772: F, t14786: F, t14802: F, t14894: F, t1559: F, t18632: F, t50312: F, t50325: F, t50328: F, t50347: F, t51014: F, t799: F, t800: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61397, t61400, t61403, t61407, t61411) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3238::<F>(t2439, t2440, t6049, t14472, t1580, t2444, t689, t136, t2457, t41011, t6048, t10504, t6071);
        let t61429 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3239::<F>(t11007, t252, t2782, t6048, t886, t14481, t1569, t2771, t40970, t40978, t40986, t40988, t41078, t50214, t50218, t50220, t50222, t50227, t50232, t50236, t61397, t61400, t61403, t61407, t61411, t865);
        let (t61430, t61437, t61441, t61448, t61471) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3240::<F>(t18805, t41066, t10995, t122, t18796, t2466, t11044, t18797, t18317, t2435, t10770, t14791, t14917, t18426, t2724, t2745, t40337, t40357, t40361, t4362, t4364, t50292, t50296, t50298, t50303, t50308, t51049, t6035);
        let t61496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3241::<F>(t45, t13312, t13396, t1490, t18281, t18367, t18372, t2251, t2258, t4328, t5860, t5862, t606, t60717, t60754, t766, t80, zeta_threshold);
        let t61517 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3242::<F>(t57, t13312, t13396, t1491, t18281, t18379, t18384, t2251, t2258, t4335, t5864, t5866, t606, t60717, t60754, t770, t83, zeta_threshold);
        let (t61519, t61532, t61544) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3243::<F>(t61496, t61517, t10871, t5977, t14931, t18477, t51123, t10811, t18471, t18451, t124, t14772, t14786, t14791, t14802, t14894, t1559, t18632, t2745, t4362, t50312, t50325, t50328, t50347, t51014, t799, t800);
    (t61429, t61430, t61437, t61441, t61448, t61471, t61519, t61532, t61544)
}
