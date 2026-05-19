//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 582/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk582<F: Float>(t2268: F, t3327: F, t3094: F, t3107: F, t3099: F, t3104: F, t471: F, t871: F, t984: F, t3114: F) -> (F, F, F) {
    let t3329 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t3327;
    let t3330 = F::new(3.0) / F::new(128.0) * t3094;
    let t3333 = t3107 / F::new(128.0);
    let t3334 = t3330 - F::new(9.0) / F::new(4096.0) * t3099 + F::new(3.0) / F::new(4096.0) * t3104 - t3333;
    let t3335 = t3334 * t471;
    let t3336 = t984 * t871;
    let t3338 = t3335 + t3336 / F::new(2.0) + t3330 - t3333 - t3114;
    (t3329, t3334, t3338)
}
