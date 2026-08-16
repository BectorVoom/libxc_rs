//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2449/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2449<F: Float>(t43002: F, t48156: F, t48158: F, t60163: F, t60168: F, t60173: F, t60192: F, t60194: F, t60202: F, t60204: F, t60274: F, t60308: F, t60310: F, t60312: F, t68545: F, t68549: F, t68552: F, t68556: F, t68563: F, t68649: F) -> F {
    let t69615 = -F::cast_from(4.0_f64) * t68545 + F::cast_from(3.0_f64) * t68549 + F::cast_from(2.0_f64) * t68552 - t68556 - t60163 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t60168 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t60173 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t68563 - t48156 + t48158 - F::cast_from(2.0_f64) * t60192 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t60194 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t60202 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t60204 - t43002 - t60274 / F::cast_from(9.0_f64) + t68649 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t60308 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t60310 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t60312;
    t69615
}
