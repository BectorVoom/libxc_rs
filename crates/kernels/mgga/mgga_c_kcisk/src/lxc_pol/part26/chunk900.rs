//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 900/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk900<F: Float>(t4384: F, t6459: F, t1572: F, t6497: F, t12261: F, t2327: F, t535: F, t240: F, t5761: F, t19104: F, t1203: F, t2097: F, t3722: F, t1528: F, t6515: F, t2285: F, t4460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21668 = t6459 * t4384;
    let t21672 = t1572 * t6497;
    let t21674 = t12261 * t2327;
    let t21675 = t535 * t21674;
    let t21710 = t240 * t5761;
    let t21720 = 0.2283111111111111111e-1 * t19104;
    let t21742 = t5761 * t1203;
    let t21748 = t2097 * t3722;
    let t21759 = t6515 * t1528;
    let t21764 = t2285 * t4460;
    (t21668, t21672, t21675, t21710, t21720, t21742, t21748, t21759, t21764)
}
