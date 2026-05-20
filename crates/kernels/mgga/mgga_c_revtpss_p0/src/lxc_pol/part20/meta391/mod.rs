//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1438;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1439;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1440;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1441;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1442;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1443;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta391<F: Float>(t907: F, t9292: F, t2435: F, t2859: F, t11166: F, t689: F, t11157: F, t11152: F, t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41338: F, t923: F, t273: F, t2881: F, t2889: F, t2897: F, t41292: F, t41299: F, t41303: F, t41307: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t41361 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1438::<F>(t907, t9292);
        let t41363 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1439::<F>(t2435, t2859);
        let t41365 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1440::<F>(t11166, t689);
        let t41367 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1441::<F>(t11157, t689);
        let t41369 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1442::<F>(t11152, t689);
        let t41371 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1443::<F>(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t41372, t41373, t41383, t41384, t41386, t41387, t41389) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1444::<F>(t41338, t41371, t923, t273, t2881, t2889, t2897, t41292, t41299, t41303, t41307, t41341, t41344, t41347, t41350, t41361, t41363, t41369);
    (t41361, t41363, t41365, t41367, t41369, t41372, t41373, t41383, t41384, t41386, t41387, t41389)
}
