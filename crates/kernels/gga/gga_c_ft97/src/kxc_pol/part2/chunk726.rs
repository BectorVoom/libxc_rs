//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 726/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk726<F: Float>(t12939: F, t143: F, t160: F, t1017: F, t2075: F, t167: F, t2185: F, t2157: F, t574: F, t605: F, t3565: F, t558: F, t3541: F, t376: F, t89: F, t1882: F, t3452: F) -> (F, F, F, F, F, F) {
    let t12941 = t143 * t12939 * t160;
    let t12945 = t1017 * t2075;
    let t12947 = t2185 * t167 * t12945;
    let t12950 = t1017 * t2157;
    let t12952 = t574 * t605 * t12950;
    let t12956 = t3565 * t558;
    let t12958 = t574 * t605 * t12956;
    let t12963 = 2.0 / 9.0 * t89 * t376 * t3541;
    let t12965 = 4.0 / 9.0 * t1882 * t3452;
    (t12941, t12947, t12952, t12958, t12963, t12965)
}
