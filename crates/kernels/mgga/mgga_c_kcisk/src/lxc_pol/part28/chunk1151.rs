//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1151/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1151<F: Float>(t33162: F, t9725: F, t9721: F, t9736: F, t4998: F, t9742: F, t9740: F, t11986: F, t79: F) -> (F, F, F, F, F) {
    let t33163 = t9725 * t33162;
    let t33165 = t9721 * t9736;
    let t33167 = t4998 * t9742;
    let t33168 = t9740 * t33167;
    let t33176 = t11986 * t79;
    (t33163, t33165, t33167, t33168, t33176)
}
