//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3897/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3897<F: Float>(t1358: F, t212: F, t22307: F, t689: F, t5774: F, t14114: F, t14216: F, t14145: F, t2482: F, t4114: F, t6843: F, t1432: F, t22379: F, t2470: F) -> (F, F, F, F, F) {
    let t74853 = t689 * t212 * t22307 * t1358;
    let t74855 = t5774 * t5774;
    let t74862 = t14114 * t14216;
    let t74866 = t2482 * t4114 * t6843 * t14145;
    let t74873 = t1432 * t22379 * t2470;
    (t74853, t74855, t74862, t74866, t74873)
}
