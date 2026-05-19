//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 368/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk368<F: Float>(t1570: F, t179: F, t178: F, t1638: F, t342: F, t630: F, t657: F, t420: F, t703: F) -> (F, F, F, F, F, F) {
    let t2271 = t179 * t1570;
    let t2280 = t178 * t178;
    let t2281 = F::new(1.0) / t2280;
    let t2289 = F::cast_from(0.19257444444444444444e0_f64) * t1638;
    let t2319 = t342 * t630 * t657 / F::new(12.0);
    let t2320 = t420 * t703;
    (t2271, t2280, t2281, t2289, t2319, t2320)
}
