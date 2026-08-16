//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 645/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk645<F: Float>(t1060: F, t3613: F, t783: F, t3283: F, t3286: F, t3289: F, t3318: F, t3323: F, t3346: F, t3586: F, t3589: F, t3592: F, t3595: F, t3598: F, t3600: F, t3604: F, t3608: F, t3611: F) -> F {
    let t3615 = t783 * t3613 * t1060;
    let t3617 = -t3283 + t3286 - t3289 - F::cast_from(0.54878743191129263322e-1_f64) * t3586 - F::cast_from(0.27439371595564631661e-1_f64) * t3589 - F::cast_from(0.43341108700271342816e-1_f64) * t3592 - F::cast_from(0.13002332610081402845e0_f64) * t3595 - F::cast_from(0.43341108700271342816e-1_f64) * t3598 + F::cast_from(0.43341108700271342816e-1_f64) * t3600 - t3318 + t3323 + F::cast_from(0.21831846657716620896e-2_f64) * t3604 + F::cast_from(0.65495539973149862688e-2_f64) * t3608 + F::cast_from(0.21831846657716620896e-2_f64) * t3611 - F::cast_from(0.21831846657716620896e-2_f64) * t3615 - t3346;
    t3617
}
