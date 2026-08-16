//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1438;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1439;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1440;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1441;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1442;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1443;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta391(t907: f64, t9292: f64, t2435: f64, t2859: f64, t11166: f64, t689: f64, t11157: f64, t11152: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41338: f64, t923: f64, t273: f64, t2881: f64, t2889: f64, t2897: f64, t41292: f64, t41299: f64, t41303: f64, t41307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t41361 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1438(t907, t9292);
        let t41363 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1439(t2435, t2859);
        let t41365 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1440(t11166, t689);
        let t41367 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1441(t11157, t689);
        let t41369 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1442(t11152, t689);
        let t41371 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1443(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t41372, t41373, t41383, t41384, t41386, t41387, t41389) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1444(t41338, t41371, t923, t273, t2881, t2889, t2897, t41292, t41299, t41303, t41307, t41341, t41344, t41347, t41350, t41361, t41363, t41369);
    (t41361, t41363, t41365, t41367, t41369, t41372, t41373, t41383, t41384, t41386, t41387, t41389)
}
