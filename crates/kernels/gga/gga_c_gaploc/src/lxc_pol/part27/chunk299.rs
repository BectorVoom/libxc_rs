//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 299/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk299<F: Float>(t1230: F, t1255: F, t1257: F, t157: F, t470: F, t471: F, t64: F, t90: F) -> F {
    let t1265 = t1257 * t471 - F::new(4.0) / F::new(3.0) * t470 * t64 + F::new(7.0) / F::new(96.0) * t1230 - F::new(7.0) / F::new(288.0) * t1255 + F::new(4.0) / F::new(3.0) * t157 * t90;
    t1265
}
