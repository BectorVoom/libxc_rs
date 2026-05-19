//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1097/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1097<F: Float>(t5241: F, t935: F, t7419: F, t9805: F, t22315: F, t9890: F, t3294: F, t739: F, t7803: F, t7805: F, t7383: F, t948: F, t9796: F) -> (F, F, F, F) {
    let t28286 = t5241 * t935;
    let t28289 = F::cast_from(0.10352590007558602413e2_f64) * t9805 * t28286 * t7419;
    let t28290 = t22315 * t9890;
    let t28296 = t7803 * t739 * t3294 * t7805;
    let t28307 = t9796 * t948 * t7383;
    (t28289, t28290, t28296, t28307)
}
