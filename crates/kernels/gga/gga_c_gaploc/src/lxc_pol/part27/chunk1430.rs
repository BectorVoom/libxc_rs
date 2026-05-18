//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1430/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1430<F: Float>(t28563: F, t28566: F, t28569: F, t33257: F, t33259: F, t33261: F, t33269: F, t33271: F, t33274: F, t33282: F, t33284: F, t33292: F, t33297: F, t33299: F, t33311: F, t33313: F) -> F {
    let t39106 = -t33257 - t33259 + t33261 + t33269 + t33271 + t33274 + F::new(0.76685851907841499354e0) * t28563 + F::new(0.76685851907841499354e0) * t28566 + F::new(0.38342925953920749677e0) * t28569 - t33282 - t33284 - t33292 - t33297 - t33299 + t33311 + t33313;
    t39106
}
