//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1378/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1378<F: Float>(t1: F, t38285: F, t544: F, t1424: F, t30246: F, t30250: F, t30253: F, t30260: F, t30263: F, t30265: F, t30288: F, t30294: F, t34249: F, t34251: F, t34253: F, t34256: F, t34258: F, t34260: F, t34261: F) -> F {
    let t38486 = t544 * t38285 * t1;
    let t38489 = -t34249 + t34251 + t34253 + t34256 + t34258 - F::new(0.10224780254378866581e1) * t30246 - F::new(0.76685851907841499354e0) * t30250 + t30253 - F::new(0.38342925953920749677e0) * t30260 - t34260 + t30263 - t30265 - t30288 + t30294 - F::new(0.79445533226334281486e-1) * t38486 * t1424 + t34261;
    t38489
}
