//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3197;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3198;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3199;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3200;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta851(t3625: f64, t44250: f64, t5401: f64, t127: f64, t5277: f64, t12866: f64, t3630: f64, t17550: f64, t372: f64, t17352: f64, t3153: f64, t3623: f64, t53667: f64, t45619: f64, t1794: f64, t42871: f64, t3666: f64, t5390: f64, t1042: f64, t1261: f64, t12841: f64, t12842: f64, t12846: f64, t12847: f64, t13043: f64, t1469: f64, t17513: f64, t17633: f64, t17644: f64, t17661: f64, t17693: f64, t17700: f64, t21014: f64, t21017: f64, t3626: f64, t3647: f64, t3714: f64, t3720: f64, t44510: f64, t44517: f64, t44535: f64, t44786: f64, t44789: f64, t44792: f64, t5268: f64, t5302: f64, t5333: f64, t53459: f64, t53464: f64, t57622: f64, t58872: f64, t17794: f64, t3584: f64, t606: f64, t17203: f64, t3172: f64, t43766: f64, t44361: f64, t12916: f64, t17419: f64, t5340: f64, t45608: f64, t1248: f64, t12780: f64, t12788: f64, t12800: f64, t12805: f64, t12938: f64, t12956: f64, t13045: f64, t16746: f64, t17199: f64, t17232: f64, t17401: f64, t17541: f64, t17569: f64, t17654: f64, t17729: f64, t17799: f64, t20945: f64, t2251: f64, t3362: f64, t3604: f64, t44230: f64, t44260: f64, t44797: f64, t5051: f64, t5270: f64, t5299: f64, t53474: f64, t5402: f64, t5405: f64, t56903: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t58889, t58897, t58899, t58909, t58919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3197(t3625, t44250, t5401, t127, t5277, t12866, t3630, t17550, t372, t17352, t3153, t3623, t53667);
        let (t58920, t58921) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3198(t45619, t58919, t1794, t42871);
        let t58948 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3199(t3666, t5390, t1042, t1261, t12841, t12842, t12846, t12847, t12866, t13043, t1469, t17513, t17633, t17644, t17661, t17693, t17700, t21014, t21017, t3625, t3626, t3647, t3714, t372, t3720, t44510, t44517, t44535, t44786, t44789, t44792, t5268, t5302, t5333, t53459, t53464, t57622, t58872, t58889, t58897, t58899, t58909, t58920, t58921);
        let (t58960, t58969, t58975, t58983, t58997) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3200(t17794, t372, t3584, t606, t1261, t17203, t3172, t43766, t44361, t12916, t17419, t5340);
        let t59007 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3201(t45608, t58919, t1042, t1248, t1261, t12780, t12788, t12800, t12805, t12866, t12938, t12956, t13043, t13045, t16746, t17199, t17232, t17401, t17541, t17569, t17654, t17661, t17729, t17799, t20945, t2251, t3362, t3604, t3625, t3626, t3647, t3720, t44230, t44260, t44797, t5051, t5270, t5299, t53474, t5402, t5405, t56903, t58921, t58960, t58969, t58975, t58983, t58997);
    (t58909, t58919, t58921, t58948, t58969, t59007)
}
