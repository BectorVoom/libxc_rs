//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1061/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1061<F: Float>(t27636: F, t7429: F, t6176: F, t1889: F, t6207: F, t6159: F, t2256: F, t23036: F, t1650: F, t2104: F, t27584: F, t4440: F, t6944: F, t7979: F, t1600: F, t6937: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29568 = t27636 * t7429;
    let t29569 = t6176 * t29568;
    let t29574 = t6207 * t1889;
    let t29575 = t6159 * t29574;
    let t29578 = t23036 * t2256;
    let t29581 = t1650 * t2104;
    let t29582 = t27584 * t29581;
    let t29583 = t4440 * t29582;
    let t29590 = t7979 * t6944;
    let t29591 = t1600 * t29590;
    let t29594 = t7979 * t6937;
    (t29568, t29569, t29574, t29575, t29578, t29582, t29583, t29590, t29591, t29594)
}
