//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1910;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta565<F: Float>(t1873: F, t94519: F, t26004: F, t5690: F, t13951: F, t2018: F, t807: F, t25240: F, t3964: F, t5617: F, t543: F, t97870: F, t786: F, t97961: F, t1444: F, t5675: F, t25898: F, t98040: F, t1907: F, t3889: F, t25081: F, t7897: F, t1518: F, t2319: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98260, t98269, t98281, t98285, t98299) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1910::<F>(t1873, t94519, t26004, t5690, t13951, t2018, t807, t25240, t3964, t5617, t543, t97870);
        let (t98308, t98362, t98380, t98436, t98450, t98484) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1911::<F>(t786, t97961, t1444, t5675, t25898, t98040, t1907, t3889, t25081, t7897, t1518, t2319);
    (t98260, t98269, t98281, t98285, t98299, t98308, t98362, t98380, t98436, t98450, t98484)
}
