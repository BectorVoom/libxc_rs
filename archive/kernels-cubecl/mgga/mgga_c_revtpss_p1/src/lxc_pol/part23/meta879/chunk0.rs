//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2786/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2786<F: Float>(t14114: F, t14216: F, t14145: F, t2482: F, t4114: F, t6843: F, t1432: F, t22379: F, t2470: F, t1437: F, t4104: F, t6861: F) -> (F, F, F, F) {
    let t74862 = t14114 * t14216;
    let t74866 = t2482 * t4114 * t6843 * t14145;
    let t74873 = t1432 * t22379 * t2470;
    let t74880 = t2482 * t1437 * t6861 * t4104;
    (t74862, t74866, t74873, t74880)
}
