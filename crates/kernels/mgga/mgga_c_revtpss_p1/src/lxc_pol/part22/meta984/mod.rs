//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta984 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta984<F: Float>(t18936: F, t2251: F, t141: F, t930: F, t18969: F, t698: F, t18972: F, t2258: F, t6092: F, t13312: F, t4578: F, t18281: F, t2857: F, t606: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t63236, t63238, t63240, t63242, t63244, t63246, t63248, t63250, t63253) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3334::<F>(t18936, t2251, t141, t930, t18969, t698, t18972, t2258, t6092, t13312, t4578, t18281, t2857, t606);
    (t63236, t63238, t63240, t63242, t63244, t63246, t63248, t63250, t63253)
}
