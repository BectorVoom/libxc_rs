//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 814/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk814(t2657: f64, t9501: f64, t2660: f64, t9019: f64, t2721: f64, t3103: f64, t2255: f64, t2636: f64, t9468: f64, t9474: f64, t9478: f64, t9481: f64, t9483: f64, t9486: f64, t9488: f64, t9491: f64, t9494: f64, t9499: f64) -> (f64, f64) {
    let t9502 = t9501 * t2657;
    let t9504 = t2660 * t9019;
    let t9505 = t9504 * t2657;
    let t9507 = t2721 * t3103;
    let t9508 = t2636 * t2255;
    let t9509 = t9507 * t9508;
    let t9511 = -0.12357942809624928455e-3_f64 * t9468 - 0.41193142698749761516e-5_f64 * t9474 + 0.3373480902777777778e-5_f64 * t9478 - 0.2318836277704281739e-4_f64 * t9481 - 0.10821235962619981449e-3_f64 * t9483 - 0.56273499301538336859e-7_f64 * t9486 + 0.27801896084645508334e-2_f64 * t9488 - 0.10120442708333333334e-4_f64 * t9491 - 0.10120442708333333334e-4_f64 * t9494 - 0.11101451561577199508e-4_f64 * t9499 + 0.56360603971979070047e-7_f64 * t9502 - 0.10020915386217878654e-6_f64 * t9505 + 0.27801896084645508334e-2_f64 * t9509;
    (t9504, t9511)
}
