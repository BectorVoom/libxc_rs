//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2770/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2770<F: Float>(t6880: F, t9779: F, t22062: F, t9775: F, t13845: F, t22145: F, t48100: F, t22068: F, t9765: F, t22052: F, t3989: F, t22022: F) -> (F, F, F, F, F, F) {
    let t74279 = t9779 * t6880;
    let t74281 = t9775 * t22062;
    let t74288 = t13845 * t48100 * t22145;
    let t74290 = t9765 * t22068;
    let t74292 = t3989 * t22052;
    let t74299 = t9775 * t22022;
    (t74279, t74281, t74288, t74290, t74292, t74299)
}
