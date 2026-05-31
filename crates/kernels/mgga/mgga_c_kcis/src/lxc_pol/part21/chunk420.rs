//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 420/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk420<F: Float>(t2621: F, t158: F, t157: F, t2586: F, t2589: F, t2591: F, t2595: F, t2598: F, t2601: F, t2603: F, t2606: F, t2608: F, t2610: F, t2613: F, t2616: F, t2619: F) -> (F, F, F, F) {
    let t2622 = F::cast_from(1.0_f64) / t2621;
    let t2623 = t158 * t2622;
    let t2624 = t157 * t2623;
    let t2626 = t2586 / F::cast_from(8.0_f64) - t2589 / F::cast_from(4.0_f64) - t2591 / F::cast_from(2.0_f64) + t2595 / F::cast_from(4.0_f64) + t2598 / F::cast_from(2.0_f64) - t2601 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t2603 - t2606 / F::cast_from(64.0_f64) + t2608 / F::cast_from(32.0_f64) + t2610 / F::cast_from(8.0_f64) - t2613 / F::cast_from(32.0_f64) - t2616 / F::cast_from(8.0_f64) + t2619 / F::cast_from(64.0_f64) - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t2624;
    (t2622, t2623, t2624, t2626)
}
