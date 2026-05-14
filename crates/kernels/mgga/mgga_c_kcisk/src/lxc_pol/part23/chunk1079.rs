//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1079/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1079<F: Float>(t1572: F, t6497: F, t12261: F, t2327: F, t535: F, t1580: F, t1583: F, t21644: F, t21648: F, t21654: F, t21658: F, t21662: F, t21665: F, t21668: F, t2318: F, t4378: F, t6583: F) -> (F,) {
    let t21672 = t1572 * t6497;
    let t21674 = t12261 * t2327;
    let t21675 = t535 * t21674;
    let t21679 = -0.11993859144118211476e-1 * t1580 * t21644 + 0.17990788716177317213e-1 * t1580 * t21648 + 0.17990788716177317213e-1 * t1580 * t21654 + 0.53972366148531951639e-1 * t1580 * t21658 + 0.17990788716177317213e-1 * t21662 * t1583 - 0.47975436576472845902e-1 * t21665 * t1583 + 0.59969295720591057378e-2 * t21668 + 0.14392630972941853771e0 * t1572 * t6583 + 0.47975436576472845902e-1 * t21672 + 0.59969295720591057378e-2 * t21675 + 0.5397236614853195164e-1 * t2318 * t4378;
    (t21679,)
}
