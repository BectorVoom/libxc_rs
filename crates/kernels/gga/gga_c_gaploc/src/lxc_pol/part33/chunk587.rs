//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 587/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk587<F: Float>(t2268: F, t3355: F, t2798: F, t921: F, t1016: F, t2355: F) -> (F, F, F, F) {
    let t3357 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t3355;
    let t3364 = t2798 * t921;
    let t3365 = t2355 * t1016;
    let t3366 = t1016 * t921;
    (t3357, t3364, t3365, t3366)
}
