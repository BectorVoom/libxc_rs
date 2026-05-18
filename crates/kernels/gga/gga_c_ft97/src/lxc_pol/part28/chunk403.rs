//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 403/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk403<F: Float>(t5935: F, t609: F, t144: F, t1386: F, t1882: F, t1384: F, t604: F, t379: F, t2210: F, t558: F) -> (F, F, F, F, F, F) {
    let t5936 = t5935 * t609;
    let t5937 = t144 * t5936;
    let t5941 = t1882 * t1386 / F::new(9.0);
    let t5942 = t604 * t1384;
    let t5943 = t5942 * t379;
    let t5944 = t2210 * t5943;
    let t5947 = t1384 * t558;
    (t5937, t5941, t5942, t5943, t5944, t5947)
}
