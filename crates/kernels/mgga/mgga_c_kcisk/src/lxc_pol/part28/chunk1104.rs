//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1104/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1104<F: Float>(t227: F, t24080: F, t24082: F, t24084: F, t24086: F, t24088: F, t24090: F, t24093: F, t24096: F, t24097: F, t24100: F, t24496: F, t240: F, t25273: F, t22591: F, t565: F, t15772: F, t7706: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t25274 = -t24080 + t24082 + t24084 - t24086 + t24088 - t24090 + t24093 - t24096 + t24097 - t24100 + t24496;
    let t25277 = t24080 - t24082 - t24084 + t24086 - t24088 + t24090 - t24093 + t24096 - t24097 + t24100 - t24496 + t240 * (t25273 + t25274);
    let t25289 = piecewise3(t228, 0.0, t22591);
    let t25290 = t565 * t25289;
    let t25312 = -t22591;
    let t28153 = t15772 * t7706;
    (t25277, t25289, t25290, t25312, t28153)
}
