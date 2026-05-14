//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 896/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk896<F: Float>(t1168: F, t6187: F, t2568: F, t5999: F, t6745: F, t681: F, t6839: F, t1403: F, t24429: F, t24237: F, t6749: F, t1173: F, t6062: F, t193: F, t6838: F, t771: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27924 = t6187 * t1168;
    let t27925 = t2568 * t27924;
    let t27927 = t6745 * t5999;
    let t27929 = t681 * t6839;
    let t27930 = t1403 * t27929;
    let t27934 = t24429 * t1168;
    let t27936 = t24237 * t6749;
    let t27938 = t6062 * t1173;
    let t27939 = t193 * t27938;
    let t27942 = t6838 * t771;
    (t27924, t27925, t27927, t27929, t27930, t27934, t27936, t27938, t27939, t27942)
}
