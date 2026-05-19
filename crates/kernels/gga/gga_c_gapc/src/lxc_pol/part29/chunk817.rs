//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 817/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk817<F: Float>(t2657: F, t9501: F, t2660: F, t9019: F, t2721: F, t3103: F, t2255: F, t2636: F, t9468: F, t9474: F, t9478: F, t9481: F, t9483: F, t9486: F, t9488: F, t9491: F, t9494: F, t9499: F) -> (F, F) {
    let t9502 = t9501 * t2657;
    let t9504 = t2660 * t9019;
    let t9505 = t9504 * t2657;
    let t9507 = t2721 * t3103;
    let t9508 = t2636 * t2255;
    let t9509 = t9507 * t9508;
    let t9511 = -F::cast_from(0.12357942809624928455e-3_f64) * t9468 - F::cast_from(0.41193142698749761516e-5_f64) * t9474 + F::cast_from(0.3373480902777777778e-5_f64) * t9478 - F::cast_from(0.2318836277704281739e-4_f64) * t9481 - F::cast_from(0.10821235962619981449e-3_f64) * t9483 - F::cast_from(0.56273499301538336859e-7_f64) * t9486 + F::cast_from(0.27801896084645508334e-2_f64) * t9488 - F::cast_from(0.10120442708333333334e-4_f64) * t9491 - F::cast_from(0.10120442708333333334e-4_f64) * t9494 - F::cast_from(0.11101451561577199508e-4_f64) * t9499 + F::cast_from(0.56360603971979070047e-7_f64) * t9502 - F::cast_from(0.10020915386217878654e-6_f64) * t9505 + F::cast_from(0.27801896084645508334e-2_f64) * t9509;
    (t9504, t9511)
}
