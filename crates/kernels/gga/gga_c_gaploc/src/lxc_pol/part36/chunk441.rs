//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 441/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk441<F: Float>(t123: F, t5241: F, t1980: F, t2032: F, t2084: F, t296: F, t120: F, t19: F, t320: F, t2088: F, t298: F) -> (F, F, F, F, F, F) {
    let t5641 = t5241 * t123;
    let t5676 = t1980 * t2032;
    let t5745 = t2084 * t296;
    let t5746 = t120 * t5745;
    let t5747 = t5746 * t19;
    let t5748 = t320 * t5747;
    let t5750 = F::new(1.0) / t2088 / t298;
    (t5641, t5676, t5745, t5747, t5748, t5750)
}
