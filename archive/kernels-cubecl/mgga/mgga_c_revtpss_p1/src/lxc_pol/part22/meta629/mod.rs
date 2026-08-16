//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2545;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2546;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta629<F: Float>(t127: F, t371: F, t6337: F, t3205: F, t6276: F, t1025: F, t4845: F, t4858: F, t3172: F, t6307: F, t3150: F, t4820: F, t4879: F, t11947: F, t15745: F, t16134: F, t16160: F, t16190: F, t1665: F, t1671: F, t3188: F, t6327: F, t6339: F, t1592: F, t999: F, t1045: F, t15691: F, t1066: F, t18946: F, t247: F, t11725: F, t6092: F, t1063: F, t3109: F, t6100: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20016, t20017, t20020, t20021, t20025, t20029, t20030, t20034) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2545::<F>(t127, t371, t6337, t3205, t6276, t1025, t4845, t4858, t3172, t6307, t3150, t4820, t4879);
        let t20036 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2546::<F>(t11947, t15745, t16134, t16160, t16190, t1665, t1671, t20017, t20021, t20025, t20030, t20034, t3188, t6327, t6339);
        let (t20038, t20039, t20040, t20046, t20050, t20051, t20054) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2547::<F>(t1592, t999, t1045, t15691, t1066, t18946, t247, t11725, t6092, t1063, t3109, t6100);
    (t20016, t20020, t20029, t20036, t20038, t20039, t20040, t20046, t20050, t20051, t20054)
}
