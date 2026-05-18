//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 754/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk754<F: Float>(t10194: F, t113: F, t2956: F, t909: F, t4381: F, t2252: F, t342: F, t784: F, t1526: F, t2640: F, t9483: F, t2644: F, t630: F) -> (F, F, F, F, F, F) {
    let t10195 = t10194 * t113;
    let t10198 = t2956 * t909;
    let t10199 = t10198 * t4381;
    let t10207 = t342 * t2252 * t784 / F::new(18.0);
    let t10209 = t1526 * t9483 * t2640;
    let t10212 = t342 * t630 * t2644;
    (t10195, t10198, t10199, t10207, t10209, t10212)
}
