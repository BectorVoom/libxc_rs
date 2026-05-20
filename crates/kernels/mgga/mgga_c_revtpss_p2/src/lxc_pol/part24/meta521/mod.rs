//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1549;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1550;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta521<F: Float>(t20816: F, t5293: F, t24611: F, t3172: F, t3711: F, t24252: F, t300: F, t17529: F, t20786: F, t21102: F, t5265: F, t5274: F, t13042: F, t24663: F, t12910: F, t12916: F, t24740: F, t21143: F, t5378: F, t21192: F, t5391: F, t21107: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82338, t82351, t82389, t82434, t82441, t82457) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1549::<F>(t20816, t5293, t24611, t3172, t3711, t24252, t300, t17529, t20786, t21102, t5265, t5274);
        let (t82469, t82491, t82534, t82536, t82550) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1550::<F>(t13042, t24663, t3172, t12910, t12916, t24740, t21143, t5378, t21192, t5391, t21107, t5265);
    (t82338, t82351, t82389, t82434, t82441, t82457, t82469, t82491, t82534, t82536, t82550)
}
