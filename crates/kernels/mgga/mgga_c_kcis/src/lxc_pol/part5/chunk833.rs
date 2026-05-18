//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 833/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk833<F: Float>(t381: F, t6708: F, t389: F, t1813: F, t5172: F, t1809: F, t1817: F, t388: F, t6486: F, t387: F, t3442: F, t3438: F, t6491: F) -> (F, F, F, F, F, F, F, F) {
    let t6709 = t6708 * t381;
    let t6710 = t6709 * t389;
    let t6712 = t5172 * t1813;
    let t6714 = t1809 * t1817;
    let t6716 = t388 * t6486;
    let t6717 = t387 * t6716;
    let t6718 = t3442 * t6717;
    let t6720 = t3438 * t6491;
    (t6709, t6710, t6712, t6714, t6716, t6717, t6718, t6720)
}
