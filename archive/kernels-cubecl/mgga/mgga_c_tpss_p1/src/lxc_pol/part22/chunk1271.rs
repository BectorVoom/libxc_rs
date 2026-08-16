//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1271/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1271<F: Float>(t1805: F, t226: F, t3664: F, t5577: F, t18770: F, t19781: F, t5572: F, t6337: F, t818: F, t782: F, t1708: F, t20446: F, t228: F) -> (F, F, F, F, F) {
    let t20492 = t5577 * t1805 * t3664 * t226;
    let t20494 = t18770 * t19781;
    let t20498 = t5572 * t6337 * t818;
    let t20502 = t6337 * t782 * t226;
    let t20503 = t5577 * t20502;
    let t20506 = t1708 * t228 * t20446;
    (t20492, t20494, t20498, t20503, t20506)
}
