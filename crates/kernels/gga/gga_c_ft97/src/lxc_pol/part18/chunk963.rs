//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 963/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk963<F: Float>(t22986: F, t3188: F, t25928: F, t5674: F, t379: F, t6469: F, t22958: F, t375: F, t6520: F, t89: F, t22993: F, t920: F, t1564: F, t446: F, t1882: F, t6513: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25929 = t22986 * t3188;
    let t25930 = t25928 * t25929;
    let t25931 = t5674 * t25930;
    let t25933 = t6469 * t379;
    let t25934 = t22958 * t25933;
    let t25935 = t5674 * t25934;
    let t25940 = t89 * t375 * t6520;
    let t25942 = t22993 * t920;
    let t25943 = t1564 * t25942;
    let t25944 = t446 * t25943;
    let t25946 = t1882 * t6513;
    (t25929, t25930, t25931, t25933, t25934, t25935, t25940, t25943, t25944, t25946)
}
