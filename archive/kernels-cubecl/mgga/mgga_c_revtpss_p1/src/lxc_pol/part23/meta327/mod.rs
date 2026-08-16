//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta327<F: Float>(t1392: F, t73: F, t1412: F, t5591: F, t1398: F, t1882: F, t13848: F, t3938: F, t9818: F, t9816: F, t125: F, t5658: F) -> (F, F, F, F, F, F) {
        let (t13902, t13910, t13926, t13941, t13943, t13944) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1624::<F>(t1392, t73, t1412, t5591, t1398, t1882, t13848, t3938, t9818, t9816, t125, t5658);
    (t13902, t13910, t13926, t13941, t13943, t13944)
}
