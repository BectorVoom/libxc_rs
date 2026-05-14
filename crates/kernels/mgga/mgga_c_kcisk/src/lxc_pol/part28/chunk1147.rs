//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1147/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1147<F: Float>(t1907: F, t9694: F, t2793: F, t5217: F, t5277: F, t654: F, t1930: F, t5283: F) -> (F, F, F, F, F) {
    let t33068 = t9694 * t1907;
    let t33071 = t2793 * t5217;
    let t33091 = t5277 * t654;
    let t33094 = t1930 * t654;
    let t33097 = t5283 * t654;
    (t33068, t33071, t33091, t33094, t33097)
}
