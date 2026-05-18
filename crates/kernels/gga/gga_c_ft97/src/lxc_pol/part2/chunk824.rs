//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 824/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk824<F: Float>(t12893: F, t12905: F, t12917: F, t12937: F, t143: F, t160: F, t1017: F, t2075: F, t167: F, t2185: F, t2157: F, t574: F, t605: F) -> (F, F, F, F) {
    let t12939 = t12893 + t12905 + t12917 + t12937;
    let t12941 = t143 * t12939 * t160;
    let t12945 = t1017 * t2075;
    let t12947 = t2185 * t167 * t12945;
    let t12950 = t1017 * t2157;
    let t12952 = t574 * t605 * t12950;
    (t12939, t12941, t12947, t12952)
}
