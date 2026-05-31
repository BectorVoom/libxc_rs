//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 899/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk899<F: Float>(t13928: F, t242: F, t3894: F, t8392: F, t2413: F, t3869: F, t2606: F, t2405: F, t3891: F, t3972: F, t761: F, t684: F) -> (F, F, F, F, F) {
    let t13929 = t242 * t13928;
    let t13933 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t8392 * t3894;
    let t13934 = t3869 * t2413;
    let t13935 = t2606 * t13934;
    let t13938 = t3869 * t2405;
    let t13939 = t3891 * t13938;
    let t13942 = t761 * t3972;
    let t13943 = t13942 * t684;
    (t13929, t13933, t13935, t13939, t13943)
}
