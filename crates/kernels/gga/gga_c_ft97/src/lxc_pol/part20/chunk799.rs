//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 799/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk799<F: Float>(t24898: F, t2867: F, t15369: F, t6275: F, t8392: F, t10443: F, t6274: F, t312: F, t6260: F, t684: F, t2874: F, t2413: F, t6273: F, t10688: F, t6374: F, t296: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24899 = t24898 * t2867;
    let t24900 = t15369 * t24899;
    let t24903 = t8392 * t6275;
    let t24905 = t10443 * t6274;
    let t24908 = t312 * t6260;
    let t24909 = t24908 * t684;
    let t24910 = t2874 * t24909;
    let t24913 = t6273 * t2413;
    let t24914 = t2874 * t24913;
    let t24917 = t10688 * t6374;
    let t24918 = t296 * t24917;
    (t24899, t24900, t24903, t24905, t24908, t24909, t24910, t24913, t24914, t24917, t24918)
}
