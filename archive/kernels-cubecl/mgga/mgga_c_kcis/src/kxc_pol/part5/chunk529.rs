//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 529/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk529<F: Float>(t137: F, t2584: F, t154: F, t754: F, t804: F, t809: F, t805: F, t812: F, t152: F, t2489: F, t2491: F, t774: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2585 = t2584 * t137;
    let t2586 = t2585 * t154;
    let t2588 = t804 * t754;
    let t2589 = t2588 * t809;
    let t2591 = t805 * t812;
    let t2593 = t152 * t2489;
    let t2594 = t154 * t2491;
    let t2595 = t2593 * t2594;
    let t2597 = t812 * t774;
    (t2585, t2586, t2588, t2589, t2591, t2593, t2594, t2595, t2597)
}
