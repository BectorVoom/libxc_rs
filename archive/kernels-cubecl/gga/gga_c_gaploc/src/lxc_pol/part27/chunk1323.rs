//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1323/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1323<F: Float>(t34530: F, t6703: F, t8248: F, t30762: F, t30765: F, t14626: F, t1562: F, t3410: F, t10348: F, t8158: F, t10601: F, t4372: F) -> (F, F, F, F, F, F, F) {
    let t34531 = F::cast_from(0.44688112439813033337e-1_f64) * t34530;
    let t34533 = F::cast_from(0.2780593662921699852e0_f64) * t8248 * t6703;
    let t34535 = F::cast_from(0.25561950635947166452e0_f64) * t30762;
    let t34536 = F::cast_from(0.25561950635947166452e0_f64) * t30765;
    let t34541 = F::cast_from(0.30674340763136599741e1_f64) * t1562 * t14626 * t3410;
    let t34548 = F::cast_from(0.14300195980740170668e1_f64) * t8158 * t10348;
    let t34556 = F::cast_from(0.92686455430723328401e-1_f64) * t10601 * t4372;
    (t34531, t34533, t34535, t34536, t34541, t34548, t34556)
}
