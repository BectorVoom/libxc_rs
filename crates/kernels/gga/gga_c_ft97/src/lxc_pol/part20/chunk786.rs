//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 786/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk786<F: Float>(t24737: F, t2579: F, t13885: F, t6076: F, t8392: F, t10085: F, t6075: F, t258: F, t6061: F, t684: F, t2599: F, t2413: F, t6074: F, t1451: F, t8232: F, t1882: F, t6105: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24738 = t24737 * t2579;
    let t24739 = t13885 * t24738;
    let t24742 = t8392 * t6076;
    let t24744 = t10085 * t6075;
    let t24747 = t258 * t6061;
    let t24748 = t24747 * t684;
    let t24749 = t2599 * t24748;
    let t24752 = t6074 * t2413;
    let t24753 = t2599 * t24752;
    let t24757 = 4.0 / 27.0 * t8232 * t1451;
    let t24758 = t1882 * t6105;
    (t24738, t24739, t24742, t24744, t24747, t24748, t24749, t24752, t24753, t24757, t24758)
}
