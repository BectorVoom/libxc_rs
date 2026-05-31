//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 395/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk395<F: Float>(t2586: F, t747: F, t741: F, t2441: F, t641: F, t746: F, t2561: F, t2565: F, t2569: F, t2573: F, t2577: F, t2581: F) -> (F, F, F, F, F, F) {
    let t2587 = t2586 * t747;
    let t2588 = t741 * t2587;
    let t2590 = t641 * t2441;
    let t2591 = t746 * t2590;
    let t2592 = t741 * t2591;
    let t2594 = t2561 / F::cast_from(16.0_f64) - t2565 / F::cast_from(16.0_f64) - t2569 / F::cast_from(6.0_f64) + t2573 / F::cast_from(24.0_f64) - t2577 / F::cast_from(256.0_f64) + t2581 / F::cast_from(256.0_f64) + t2588 / F::cast_from(48.0_f64) - t2592 / F::cast_from(192.0_f64);
    (t2587, t2588, t2590, t2591, t2592, t2594)
}
