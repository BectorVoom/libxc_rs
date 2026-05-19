//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1386/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1386<F: Float>(t11981: F, t1305: F, t1445: F, t1450: F, t1603: F, t30374: F, t30378: F, t30379: F, t30381: F, t30387: F, t34358: F, t34361: F, t34366: F, t34370: F, t34374: F, t34377: F, t34381: F, t34383: F, t34386: F, t3701: F, t4667: F) -> F {
    let t38535 = t34358 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t1445 * t11981 * t1305 + t34361 + t34366 - t34370 + t34374 + t34377 - t34381 + t30374 - t30378 + t34383 - F::cast_from(0.38342925953920749677e0_f64) * t30379 - F::cast_from(0.76685851907841499354e0_f64) * t30381 + F::cast_from(0.10224780254378866581e1_f64) * t30387 - t34386 + F::cast_from(0.71500979903700853338e0_f64) * t1603 * t3701 * t4667;
    t38535
}
