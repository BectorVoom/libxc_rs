//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 675/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk675<F: Float>(t2252: F, t342: F, t784: F, t1526: F, t2640: F, t9483: F, t2644: F, t630: F, t2347: F, t294: F, t2349: F, t2360: F) -> (F, F, F, F, F) {
    let t10207 = t342 * t2252 * t784 / F::new(18.0);
    let t10209 = t1526 * t9483 * t2640;
    let t10212 = t342 * t630 * t2644;
    let t10214 = t294 * t2347;
    let t10215 = t10214 * t2349;
    let t10222 = t294 * t2360;
    (t10207, t10209, t10212, t10215, t10222)
}
