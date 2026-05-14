//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1341/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1341<F: Float>(t1882: F, t27031: F, t27147: F, t379: F, t95292: F, t95293: F, t23925: F, t28: F, t3408: F, t89: F, t104175: F, t558: F, t105810: F, t105813: F, t105816: F, t105819: F, t105824: F, t96111: F, t96116: F) -> (F, F, F, F, F) {
    let t105826 = t1882 * t27031;
    let t105827 = 2.0 / 9.0 * t105826;
    let t105830 = t95292 * t95293 * t27147 * t379;
    let t105834 = t89 * t28 * t23925 * t3408;
    let t105838 = t89 * t28 * t104175 * t558;
    let t105840 = t96111 + t105810 - t105813 + t96116 + t105816 + t105819 / 3.0 + 2.0 / 9.0 * t105824 - t105827 + t105830 / 3.0 + 4.0 * t105834 + 4.0 * t105838;
    (t105826, t105830, t105834, t105838, t105840)
}
