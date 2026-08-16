//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1479;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1480;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1481;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta399(t11465: f64, t3006: f64, t3015: f64, t981: f64, t11602: f64, t3022: f64, t3329: f64, t3325: f64, t1071: f64, t3043: f64, t1076: f64, t1078: f64, t1079: f64, t1097: f64, t11123: f64, t11128: f64, t11174: f64, t11177: f64, t11178: f64, t11184: f64, t11187: f64, t11190: f64, t12040: f64, t12178: f64, t16312: f64, t16603: f64, t3047: f64, t3052: f64, t3058: f64, t3059: f64, t3063: f64, t3066: f64, t3075: f64, t3076: f64, t3261: f64, t3264: f64, t3268: f64, t3269: f64, t995: f64, t999: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64, t341: f64, t12032: f64, t342: f64, t11902: f64, t378: f64, t3046: f64, t3259: f64, t11199: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41947, t41949, t41950, t42000) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1479(t11465, t3006, t3015, t981, t11602, t3022, t3329, t3325, t1071, t3043, t1076, t1078, t1079, t1097, t11123, t11128, t11174, t11177, t11178, t11184, t11187, t11190, t12040, t12178, t16312, t16603, t3047, t3052, t3058, t3059, t3063, t3066, t3075, t3076, t3261, t3264, t3268, t3269, t995, t999);
        let (t42001, t42018) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1480(t3059, t3075, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t42031 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1481(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t42033, t42038, t42041, t42044, t42047, t42051) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1482(t341, t42018, t42031, t12032, t342, t11902, t378, t3046, t3259, t3075, t11199, t988);
    (t41947, t41949, t41950, t42000, t42001, t42033, t42038, t42041, t42044, t42047, t42051)
}
