//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1761;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1762;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1763;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta462<F: Float>(t5: F, t25167: F, t117: F, t4144: F, t9593: F, t2034: F, t2014: F, t10416: F, t1937: F, t13435: F, t2322: F, t6993: F, t196: F, t197: F, t3821: F, t2035: F, t531: F, t7311: F, t7238: F, t7312: F, t7315: F, t1310: F, t1453: F, t1932: F, t2007: F, t2320: F, t2328: F, t25078: F, t25085: F, t25092: F, t25095: F, t25096: F, t3813: F, t508: F, t649: F, t651: F, t6983: F, t7221: F, t7231: F, t2394: F, t30: F, t1962: F, t198: F, t206: F, t2411: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25168, t25169, t25177, t25178, t25180, t25182, t25184, t25186, t25188) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1761::<F>(t5, t25167, t117, t4144, t9593, t2034, t2014, t10416, t1937, t13435, t2322, t6993, t196, t197, t3821);
        let (t25190, t25191, t25194, t25197) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1762::<F>(t2035, t25188, t531, t7311, t7238, t2014, t7312, t7315, t1310, t1453, t1932, t2007, t2320, t2328, t25078, t25085, t25092, t25095, t25096, t25169, t25180, t25182, t25184, t25186, t3813, t508, t649, t651, t6983, t7221, t7231);
        let (t25198, t25206) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1763::<F>(t2394, t30, t1962, t198, t206);
        let t25207 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1764::<F>(t2411, t30);
    (t25168, t25169, t25177, t25178, t25188, t25190, t25191, t25194, t25197, t25198, t25206, t25207)
}
