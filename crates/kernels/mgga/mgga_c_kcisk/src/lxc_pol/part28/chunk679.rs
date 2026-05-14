//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 679/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk679<F: Float>(t2042: F, t2049: F, t240: F, t2666: F, t5527: F, t5532: F, t7292: F, t7294: F, t7295: F, t7298: F, t7445: F, t7654: F, t7656: F, t7659: F, t7690: F, t802: F) -> (F,) {
    let t7694 = t7292 - t7294 - t7295 + t7298 - t7445 + t240 * (-t2042 * t7690 - t2049 * t7656 - t2666 * t5527 + 2.0 * t5532 * t7659 + t7654 * t802 - t7292 + t7294 + t7295 - t7298 + t7445);
    (t7694,)
}
