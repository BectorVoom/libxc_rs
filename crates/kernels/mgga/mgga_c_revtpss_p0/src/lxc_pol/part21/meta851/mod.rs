//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta851 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3197;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3198;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3199;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3200;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta851<F: Float>(t3625: F, t44250: F, t5401: F, t127: F, t5277: F, t12866: F, t3630: F, t17550: F, t372: F, t17352: F, t3153: F, t3623: F, t53667: F, t45619: F, t1794: F, t42871: F, t3666: F, t5390: F, t1042: F, t1261: F, t12841: F, t12842: F, t12846: F, t12847: F, t13043: F, t1469: F, t17513: F, t17633: F, t17644: F, t17661: F, t17693: F, t17700: F, t21014: F, t21017: F, t3626: F, t3647: F, t3714: F, t3720: F, t44510: F, t44517: F, t44535: F, t44786: F, t44789: F, t44792: F, t5268: F, t5302: F, t5333: F, t53459: F, t53464: F, t57622: F, t58872: F, t17794: F, t3584: F, t606: F, t17203: F, t3172: F, t43766: F, t44361: F, t12916: F, t17419: F, t5340: F, t45608: F, t1248: F, t12780: F, t12788: F, t12800: F, t12805: F, t12938: F, t12956: F, t13045: F, t16746: F, t17199: F, t17232: F, t17401: F, t17541: F, t17569: F, t17654: F, t17729: F, t17799: F, t20945: F, t2251: F, t3362: F, t3604: F, t44230: F, t44260: F, t44797: F, t5051: F, t5270: F, t5299: F, t53474: F, t5402: F, t5405: F, t56903: F) -> (F, F, F, F, F, F) {
        let (t58889, t58897, t58899, t58909, t58919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3197::<F>(t3625, t44250, t5401, t127, t5277, t12866, t3630, t17550, t372, t17352, t3153, t3623, t53667);
        let (t58920, t58921) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3198::<F>(t45619, t58919, t1794, t42871);
        let t58948 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3199::<F>(t3666, t5390, t1042, t1261, t12841, t12842, t12846, t12847, t12866, t13043, t1469, t17513, t17633, t17644, t17661, t17693, t17700, t21014, t21017, t3625, t3626, t3647, t3714, t372, t3720, t44510, t44517, t44535, t44786, t44789, t44792, t5268, t5302, t5333, t53459, t53464, t57622, t58872, t58889, t58897, t58899, t58909, t58920, t58921);
        let (t58960, t58969, t58975, t58983, t58997) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3200::<F>(t17794, t372, t3584, t606, t1261, t17203, t3172, t43766, t44361, t12916, t17419, t5340);
        let t59007 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3201::<F>(t45608, t58919, t1042, t1248, t1261, t12780, t12788, t12800, t12805, t12866, t12938, t12956, t13043, t13045, t16746, t17199, t17232, t17401, t17541, t17569, t17654, t17661, t17729, t17799, t20945, t2251, t3362, t3604, t3625, t3626, t3647, t3720, t44230, t44260, t44797, t5051, t5270, t5299, t53474, t5402, t5405, t56903, t58921, t58960, t58969, t58975, t58983, t58997);
    (t58909, t58919, t58921, t58948, t58969, t59007)
}
