//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta509<F: Float>(t23492: F, t698: F, t23471: F, t23495: F, t23510: F, t23507: F, t23475: F, t23663: F, t914: F, t23798: F, t945: F, t23811: F, t964: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t77663, t77667, t77736, t77804, t77806, t77858, t78097, t78108, t78111) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1524::<F>(t23492, t698, t23471, t23495, t23510, t23507, t23475, t23663, t914, t23798, t945, t23811, t964);
    (t77663, t77667, t77736, t77804, t77806, t77858, t78097, t78108, t78111)
}
