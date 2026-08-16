//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1321/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1321(t34530: f64, t6703: f64, t8248: f64, t30762: f64, t30765: f64, t14626: f64, t1562: f64, t3410: f64, t10348: f64, t8158: f64, t10601: f64, t4372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34531 = 0.44688112439813033337e-1_f64 * t34530;
    let t34533 = 0.2780593662921699852e0_f64 * t8248 * t6703;
    let t34535 = 0.25561950635947166452e0_f64 * t30762;
    let t34536 = 0.25561950635947166452e0_f64 * t30765;
    let t34541 = 0.30674340763136599741e1_f64 * t1562 * t14626 * t3410;
    let t34548 = 0.14300195980740170668e1_f64 * t8158 * t10348;
    let t34556 = 0.92686455430723328401e-1_f64 * t10601 * t4372;
    (t34531, t34533, t34535, t34536, t34541, t34548, t34556)
}
