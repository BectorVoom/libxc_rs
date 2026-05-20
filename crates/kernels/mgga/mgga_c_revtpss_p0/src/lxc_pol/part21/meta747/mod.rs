//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2621;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta747<F: Float>(t47005: F, t47007: F, t13597: F, t2626: F, t5571: F, t9387: F, t47009: F, t47011: F, t47013: F, t13613: F, t2619: F, t9323: F, t47019: F, t39773: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t47003: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268, t48269) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2621::<F>(t47005, t47007, t13597, t2626, t5571, t9387, t47009, t47011, t47013, t13613, t2619, t9323);
        let (t48270, t48271, t48272) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2622::<F>(t48269, t47019, t39773, t39783, t39786, t39791, t39795, t39799, t47003, t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268);
    (t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268, t48270, t48271, t48272)
}
