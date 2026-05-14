//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1312/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1312<F: Float>(t112517: F, t9649: F, t3805: F, t9688: F, t9691: F, t1863: F, t5060: F, t18325: F, t32920: F, t18682: F, t25: F, t33207: F, t9724: F, t12261: F, t2804: F, t9747: F) -> (F, F, F, F, F, F, F, F) {
    let t112623 = t9649 * t112517;
    let t112661 = t3805 * t9688;
    let t112663 = t3805 * t9691;
    let t112692 = t1863 * t5060;
    let t112709 = t32920 * t18325;
    let t112761 = t25 * t18682;
    let t112765 = t9724 * t33207;
    let t112780 = t2804 * t12261 * t9747;
    (t112623, t112661, t112663, t112692, t112709, t112761, t112765, t112780)
}
