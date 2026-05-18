//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1114/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1114<F: Float>(t47243: F, t7427: F, t7573: F, t43497: F, t43500: F, t43502: F, t43511: F, t43514: F, t43516: F, t43519: F, t43523: F, t43527: F, t43529: F, t43567: F) -> F {
    let t47245 = t7427 * t7573 * t47243;
    let t47247 = -t43497 + t43500 + F::new(0.14896037479937677779e-1) * t43502 - t43511 + t43514 + F::new(0.43710935587469654631e2) * t43516 + F::new(0.29792074959875355558e-1) * t43519 + t43523 + t43527 - F::new(0.14896037479937677779e-1) * t43529 - F::new(0.62115540045351614476e2) * t47245 + t43567;
    t47247
}
