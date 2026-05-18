//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 317/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk317<F: Float>(t358: F, t487: F, t342: F, t511: F, t630: F, t142: F, t10: F, t144: F, t1542: F, t143: F, t1557: F, t378: F, t525: F) -> (F, F, F, F, F, F, F) {
    let t1910 = t487 * t358;
    let t1942 = t342 * t630 * t511 / F::new(12.0);
    let t1943 = t142 * t358;
    let t1956 = t10 * t1542 * t144;
    let t1957 = F::new(2.0) / F::new(27.0) * t1956;
    let t1964 = t143 * t1557;
    let t1969 = t378 * t525;
    (t1910, t1942, t1943, t1956, t1957, t1964, t1969)
}
