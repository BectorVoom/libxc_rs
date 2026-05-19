//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 713/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk713<F: Float>(t2258: F, t358: F, t68: F, t8076: F, t2993: F, t7705: F, t419: F, t173: F, t1736: F, t2984: F, t11034: F, t3088: F) -> (F, F, F, F, F) {
    let t11253 = t2258 * t358;
    let t11255 = t68 * t8076 * t11253;
    let t11259 = t7705 * t2993;
    let t11260 = t419 * t11259;
    let t11262 = t173 * t1736;
    let t11263 = t11262 * t2984;
    let t11264 = t419 * t11263;
    let t11265 = F::cast_from(0.56749874115226337448e-2_f64) * t11264;
    let t11266 = t3088 * t11034;
    (t11255, t11260, t11264, t11265, t11266)
}
