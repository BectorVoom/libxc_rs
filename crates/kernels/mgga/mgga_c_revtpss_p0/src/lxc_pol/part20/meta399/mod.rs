//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1479;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1480;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1481;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta399<F: Float>(t11465: F, t3006: F, t3015: F, t981: F, t11602: F, t3022: F, t3329: F, t3325: F, t1071: F, t3043: F, t1076: F, t1078: F, t1079: F, t1097: F, t11123: F, t11128: F, t11174: F, t11177: F, t11178: F, t11184: F, t11187: F, t11190: F, t12040: F, t12178: F, t16312: F, t16603: F, t3047: F, t3052: F, t3058: F, t3059: F, t3063: F, t3066: F, t3075: F, t3076: F, t3261: F, t3264: F, t3268: F, t3269: F, t995: F, t999: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41361: F, t41363: F, t41365: F, t41367: F, t41369: F, t341: F, t12032: F, t342: F, t11902: F, t378: F, t3046: F, t3259: F, t11199: F, t988: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t41947, t41949, t41950, t42000) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1479::<F>(t11465, t3006, t3015, t981, t11602, t3022, t3329, t3325, t1071, t3043, t1076, t1078, t1079, t1097, t11123, t11128, t11174, t11177, t11178, t11184, t11187, t11190, t12040, t12178, t16312, t16603, t3047, t3052, t3058, t3059, t3063, t3066, t3075, t3076, t3261, t3264, t3268, t3269, t995, t999);
        let (t42001, t42018) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1480::<F>(t3059, t3075, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t42031 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1481::<F>(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t42033, t42038, t42041, t42044, t42047, t42051) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1482::<F>(t341, t42018, t42031, t12032, t342, t11902, t378, t3046, t3259, t3075, t11199, t988);
    (t41947, t41949, t41950, t42000, t42001, t42033, t42038, t42041, t42044, t42047, t42051)
}
