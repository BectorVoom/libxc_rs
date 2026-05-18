//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 295/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk295<F: Float>(t1233: F, t1236: F, t143: F, t463: F) -> (F, F, F, F) {
    let t1237 = t1233 * t1236;
    let t1238 = t143 * t143;
    let t1240 = F::new(1.0) / t1238 / t143;
    let t1242 = t1240 * M_PI * t463;
    (t1237, t1238, t1240, t1242)
}
