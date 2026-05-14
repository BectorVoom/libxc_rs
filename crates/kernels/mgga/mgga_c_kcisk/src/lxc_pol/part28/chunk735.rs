//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 735/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk735<F: Float>(t2014: F, t7718: F, t1775: F, t5486: F, t7715: F, t5006: F, t2642: F) -> (F, F, F, F, F) {
    let t9168 = t2014 * t7718;
    let t9169 = t1775 * t9168;
    let t9172 = t5486 * t7715;
    let t9173 = t5006 * t9172;
    let t9176 = t2642 * t2642;
    (t9168, t9169, t9172, t9173, t9176)
}
