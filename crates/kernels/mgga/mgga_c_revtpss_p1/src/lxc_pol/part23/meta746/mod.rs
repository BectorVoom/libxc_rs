//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta746 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2530;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2531;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2532;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2533;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2534;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta746<F: Float>(t2439: F, t4625: F, t4622: F, t123: F, t127: F, t159: F, t1065: F, t11150: F, t11144: F, t3181: F, t2435: F, t4584: F, t1593: F, t9292: F, t138: F, t140: F, t240: F, t2852: F, t346: F, t4580: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51913, t51914, t51915, t51957) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2530::<F>(t2439, t4625, t4622, t123, t127, t159);
        let (t51958, t51963, t51973) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2531::<F>(t1065, t11150, t11144, t3181, t2435, t4584);
        let (t51974, t51978) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2532::<F>(t51973, t1593, t9292);
        let t52011 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2533::<F>(t138, t140, t240);
        let (t52018, t52035) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2534::<F>(t2852, t346, t2435, t4580);
    (t51913, t51914, t51915, t51957, t51958, t51963, t51973, t51974, t51978, t52011, t52018, t52035)
}
