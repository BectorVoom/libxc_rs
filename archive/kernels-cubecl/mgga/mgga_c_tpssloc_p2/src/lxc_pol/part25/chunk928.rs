//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 928/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk928<F: Float>(t1254: F, t3637: F, t3639: F, t500: F, t11405: F, t11409: F, t11426: F, t11429: F, t11472: F, t11480: F, t11482: F, t11484: F, t11631: F, t11636: F, t11940: F, t1256: F, t193: F, t336: F, t3633: F, t3640: F, t4700: F) -> F {
    let t11944 = t3637 * t1254;
    let t11947 = F::cast_from(1.0_f64) / t3639 / t500;
    let t11955 = t11940 * t1256 * t193 * t336 + F::cast_from(2.0_f64) * t11944 * t11947 * t193 * t336 - F::cast_from(3.0_f64) * t1254 * t3633 * t3640 * t4700 - t11405 + t11409 - t11426 + t11429 - t11472 - t11480 - t11482 - t11484 + t11631 - t11636;
    t11955
}
