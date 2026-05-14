//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 514/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk514<F: Float>(t2586: F, t2589: F, t2591: F, t2595: F, t2598: F, t2601: F, t2603: F, t2606: F, t2608: F, t2610: F, t2613: F, t2616: F, t2619: F, t2624: F, t783: F, t171: F) -> (F, F, F) {
    let t2626 = t2586 / 8.0 - t2589 / 4.0 - t2591 / 2.0 + t2595 / 4.0 + t2598 / 2.0 - t2601 / 8.0 + 3.0 / 4.0 * t2603 - t2606 / 64.0 + t2608 / 32.0 + t2610 / 8.0 - t2613 / 32.0 - t2616 / 8.0 + t2619 / 64.0 - 5.0 / 16.0 * t2624;
    let t2627 = t783 * t2626;
    let t2628 = t171 * t171;
    (t2626, t2627, t2628)
}
