//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1386/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1386(t11981: f64, t11986: f64, t12054: f64, t12093: f64, t1328: f64, t1445: f64, t30574: f64, t30578: f64, t30606: f64, t30629: f64, t30631: f64, t30633: f64, t30644: f64, t30647: f64, t30650: f64, t34436: f64, t34442: f64, t38272: f64, t4819: f64, t4820: f64, t4849: f64, t574: f64, t597: f64, t6820: f64) -> f64 {
    let t38559 = -t34436 - 0.10224780254378866581e1_f64 * t30574 + t30578 - 0.79445533226334281486e-1_f64 * t4819 * t4820 * t38272 + t34442 + 0.10224780254378866581e1_f64 * t30606 + t30629 - t30631 + t30633 - 0.1022478025437886658e1_f64 * t4849 * t12093 + t30644 - t30647 + t30650 + 0.43710935587469654631e2_f64 * t597 * t1445 * t11981 * t1328 - 0.92023022289409799224e1_f64 * t574 * t1445 * t11986 * t1328 - 0.10725146985555128001e1_f64 * t12054 * t6820;
    t38559
}
