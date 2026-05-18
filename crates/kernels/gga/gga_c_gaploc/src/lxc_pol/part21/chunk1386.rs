//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1386/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1386<F: Float>(t11981: F, t11986: F, t12054: F, t12093: F, t1328: F, t1445: F, t30574: F, t30578: F, t30606: F, t30629: F, t30631: F, t30633: F, t30644: F, t30647: F, t30650: F, t34436: F, t34442: F, t38272: F, t4819: F, t4820: F, t4849: F, t574: F, t597: F, t6820: F) -> F {
    let t38559 = -t34436 - F::new(0.10224780254378866581e1) * t30574 + t30578 - F::new(0.79445533226334281486e-1) * t4819 * t4820 * t38272 + t34442 + F::new(0.10224780254378866581e1) * t30606 + t30629 - t30631 + t30633 - F::new(0.1022478025437886658e1) * t4849 * t12093 + t30644 - t30647 + t30650 + F::new(0.43710935587469654631e2) * t597 * t1445 * t11981 * t1328 - F::new(0.92023022289409799224e1) * t574 * t1445 * t11986 * t1328 - F::new(0.10725146985555128001e1) * t12054 * t6820;
    t38559
}
