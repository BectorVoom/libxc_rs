//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 646/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk646<F: Float>(t170: F, t180: F, t8715: F, t645: F, t8640: F, t2252: F, t342: F, t511: F, t1526: F, t1944: F, t7705: F, t1948: F, t630: F) -> (F, F, F, F, F) {
    let t8718 = F::new(20.0) / F::new(27.0) * t170 * t8715 * t180;
    let t8719 = t8640 * t645;
    let t8759 = t342 * t2252 * t511 / F::new(18.0);
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    (t8718, t8719, t8759, t8761, t8764)
}
