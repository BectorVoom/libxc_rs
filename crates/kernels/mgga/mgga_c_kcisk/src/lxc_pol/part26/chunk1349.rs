//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1349/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1349<F: Float>(t4170: F, t6394: F, t9848: F, t118647: F, t118650: F, t118653: F, t118655: F, t118656: F, t118659: F, t118661: F, t118662: F, t118664: F, t118667: F, t118669: F, t118671: F, t118674: F, t118677: F, t118680: F, t118682: F, t119709: F) -> (F, F) {
    let t119712 = 4.0 * t4170 * t9848 * t6394;
    let t119713 = t118647 + t118650 + t118653 + t118655 - t118656 - t118659 - t118661 - t118662 - t118664 + t118667 - t118669 + t118671 - t118674 + t118677 + t118680 - t118682 + t119709 + t119712;
    (t119712, t119713)
}
