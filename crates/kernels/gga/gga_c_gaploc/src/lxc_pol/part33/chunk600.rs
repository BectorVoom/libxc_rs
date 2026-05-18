//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 600/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk600<F: Float>(t1960: F, t3459: F, t3040: F, t955: F, t2976: F, t959: F, t1645: F, t948: F) -> (F, F, F, F) {
    let t3461 = F::new(2.0) * t1960 * t3459;
    let t3463 = F::new(0.35750489951850426669e0) * t955 * t3040;
    let t3468 = t2976 * t959;
    let t3469 = F::new(0.14896037479937677779e-1) * t3468;
    let t3470 = t1645 * t948;
    (t3461, t3463, t3469, t3470)
}
