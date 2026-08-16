//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 795/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk795(t12512: f64, t139: f64, t527: f64, t1014: f64, t1992: f64, t3347: f64, t3380: f64, t11260: f64, t11255: f64, t11264: f64, t11267: f64, t11271: f64, t11275: f64, t11278: f64, t11282: f64, t11284: f64, t8074: f64, t8086: f64, t8914: f64) -> (f64, f64, f64, f64) {
    let t12513 = t139 * t12512;
    let t12514 = t527 * t12513;
    let t12516 = t1992 * t1014;
    let t12522 = t3347 * t3380;
    let t12527 = 0.22226000364197530866e-1_f64 * t11260;
    let t12535 = 0.33339000546296296298e-1_f64 * t11255 + 0.59269334304526748975e-1_f64 * t8074 + t8914 + 0.16299066933744855968e0_f64 * t8086 - t12527 + 0.14817333576131687243e-1_f64 * t11264 + 0.22226000364197530865e-1_f64 * t11267 + 0.51860667516460905352e-1_f64 * t11271 + 0.88904001456790123461e-1_f64 * t11275 - 0.33339000546296296298e-1_f64 * t11278 - 0.13335600218518518519e0_f64 * t11282 + 0.17780800291358024692e0_f64 * t11284;
    (t12514, t12516, t12522, t12535)
}
