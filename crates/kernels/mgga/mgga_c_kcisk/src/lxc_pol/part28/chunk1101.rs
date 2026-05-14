//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1101/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1101<F: Float>(t24183: F, t24185: F, t24187: F, t24191: F, t24193: F, t24195: F, t24197: F, t24200: F, t24204: F, t24206: F, t24209: F, t24246: F, t24249: F, t24252: F, t24255: F, t24258: F, t24261: F, t24264: F, t24266: F, t24270: F, t24273: F, t24276: F) -> (F, F) {
    let t25219 = -0.53958333333333333332e-1 * t24183 + 0.625e-1 * t24185 + 0.20234375e-1 * t24187 + 0.41666666666666666667e-1 * t24191 + 0.26979166666666666667e-1 * t24193 + 0.125e0 * t24195 + 0.1875e0 * t24197 - 0.45564814814814814815e0 * t24200 - 0.9375e-1 * t24204 - 0.33333333333333333334e0 * t24206 + 0.13489583333333333333e-1 * t24209;
    let t25244 = 0.25e0 * t24246 + 0.34173611111111111111e0 * t24249 - 0.625e-1 * t24252 - 0.20234375e-1 * t24255 + 0.1875e0 * t24258 - 1.0 * t24261 - 0.89930555555555555553e-2 * t24264 - 0.9375e-1 * t24266 - 0.13489583333333333333e-1 * t24270 + 0.26979166666666666666e-1 * t24273 + 0.89930555555555555553e-2 * t24276;
    (t25219, t25244)
}
