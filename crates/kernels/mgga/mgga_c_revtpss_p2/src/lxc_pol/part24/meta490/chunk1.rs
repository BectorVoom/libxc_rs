//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1486/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1486<F: Float>(t6880: F, t9779: F, t22062: F, t9775: F, t22068: F, t9765: F, t22022: F, t22061: F, t808: F, t9845: F, t22182: F, t47215: F) -> (F, F, F, F, F, F) {
    let t74279 = t9779 * t6880;
    let t74281 = t9775 * t22062;
    let t74290 = t9765 * t22068;
    let t74299 = t9775 * t22022;
    let t74304 = t9845 * t808 * t22061;
    let t74322 = t47215 * t22182;
    (t74279, t74281, t74290, t74299, t74304, t74322)
}
