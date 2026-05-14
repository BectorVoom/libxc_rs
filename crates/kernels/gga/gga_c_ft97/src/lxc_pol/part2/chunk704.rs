//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 704/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk704<F: Float>(t12512: F, t139: F, t527: F, t1014: F, t1992: F, t3347: F, t3380: F, t11260: F, t11255: F, t11264: F, t11267: F, t11271: F, t11275: F, t11278: F, t11282: F, t11284: F, t8074: F, t8086: F, t8914: F) -> (F, F, F, F) {
    let t12513 = t139 * t12512;
    let t12514 = t527 * t12513;
    let t12516 = t1992 * t1014;
    let t12522 = t3347 * t3380;
    let t12527 = 0.22226000364197530866e-1 * t11260;
    let t12535 = 0.33339000546296296298e-1 * t11255 + 0.59269334304526748975e-1 * t8074 + t8914 + 0.16299066933744855968e0 * t8086 - t12527 + 0.14817333576131687243e-1 * t11264 + 0.22226000364197530865e-1 * t11267 + 0.51860667516460905352e-1 * t11271 + 0.88904001456790123461e-1 * t11275 - 0.33339000546296296298e-1 * t11278 - 0.13335600218518518519e0 * t11282 + 0.17780800291358024692e0 * t11284;
    (t12514, t12516, t12522, t12535)
}
