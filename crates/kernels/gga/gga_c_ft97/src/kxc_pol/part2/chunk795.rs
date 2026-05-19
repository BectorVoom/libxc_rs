//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 795/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk795<F: Float>(t12512: F, t139: F, t527: F, t1014: F, t1992: F, t3347: F, t3380: F, t11260: F, t11255: F, t11264: F, t11267: F, t11271: F, t11275: F, t11278: F, t11282: F, t11284: F, t8074: F, t8086: F, t8914: F) -> (F, F, F, F) {
    let t12513 = t139 * t12512;
    let t12514 = t527 * t12513;
    let t12516 = t1992 * t1014;
    let t12522 = t3347 * t3380;
    let t12527 = F::cast_from(0.22226000364197530866e-1_f64) * t11260;
    let t12535 = F::cast_from(0.33339000546296296298e-1_f64) * t11255 + F::cast_from(0.59269334304526748975e-1_f64) * t8074 + t8914 + F::cast_from(0.16299066933744855968e0_f64) * t8086 - t12527 + F::cast_from(0.14817333576131687243e-1_f64) * t11264 + F::cast_from(0.22226000364197530865e-1_f64) * t11267 + F::cast_from(0.51860667516460905352e-1_f64) * t11271 + F::cast_from(0.88904001456790123461e-1_f64) * t11275 - F::cast_from(0.33339000546296296298e-1_f64) * t11278 - F::cast_from(0.13335600218518518519e0_f64) * t11282 + F::cast_from(0.17780800291358024692e0_f64) * t11284;
    (t12514, t12516, t12522, t12535)
}
