//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 392/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk392<F: Float>(t2560: F, t736: F, t2527: F, t719: F, t735: F, t1935: F, t2454: F, t41: F) -> (F, F, F, F, F) {
    let t2561 = t2560 * t736;
    let t2563 = t719 * t2527;
    let t2564 = t735 * t2563;
    let t2565 = t1935 * t2564;
    let t2567 = t2454 * t41;
    (t2561, t2563, t2564, t2565, t2567)
}
