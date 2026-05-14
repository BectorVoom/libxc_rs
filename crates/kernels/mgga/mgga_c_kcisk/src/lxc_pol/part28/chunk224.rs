//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 224/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk224<F: Float>(t222: F, t167: F, t220: F, t143: F, t224: F, zeta_threshold: F) -> (F, F, F, F) {
    let t223 = t222 <= zeta_threshold;
    let t1055 = t220 * t167;
    let t1056 = t143 - t1055;
    let t1059 = piecewise3(t223, 0.0, 4.0 / 3.0 * t224 * t1056);
    let t1060 = -t1056;
    (t1055, t1056, t1059, t1060)
}
