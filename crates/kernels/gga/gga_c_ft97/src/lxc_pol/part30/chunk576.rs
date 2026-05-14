//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 576/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk576<F: Float>(t193: F, t27907: F, t13927: F, t6175: F, t24412: F, t3864: F, t681: F, t6843: F, t1168: F, t6187: F, t2568: F, t6839: F, t24429: F, t1173: F, t6062: F, t6838: F, t771: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27908 = t193 * t27907;
    let t27911 = t13927 * t6175;
    let t27913 = t24412 * t3864;
    let t27915 = t681 * t6843;
    let t27924 = t6187 * t1168;
    let t27925 = t2568 * t27924;
    let t27929 = t681 * t6839;
    let t27934 = t24429 * t1168;
    let t27938 = t6062 * t1173;
    let t27939 = t193 * t27938;
    let t27942 = t6838 * t771;
    (t27908, t27911, t27913, t27915, t27924, t27925, t27929, t27934, t27939, t27942)
}
