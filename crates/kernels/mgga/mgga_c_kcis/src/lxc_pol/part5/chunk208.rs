//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 208/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk208<F: Float>(t661: F, t662: F, t646: F, t2: F, t45: F, t56: F, t649: F, t88: F, t47: F, t52: F) -> (F, F, F, F, F, F, F) {
    let t663 = t661 * t662;
    let t665 = 1.0 * t646 * t663;
    let t666 = t45 * t2;
    let t668 = t649 * t88 * t56;
    let t671 = t45 * t47;
    let t672 = t52 * t52;
    let t673 = 1.0 / t672;
    (t663, t665, t666, t668, t671, t672, t673)
}
