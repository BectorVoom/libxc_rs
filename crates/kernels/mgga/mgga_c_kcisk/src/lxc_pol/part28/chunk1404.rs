//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1404/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1404<F: Float>(t24480: F, t9704: F, t34316: F, t7304: F, t33091: F, t9047: F, t122239: F, t122241: F, t122243: F, t122245: F, t122248: F, t122250: F, t122252: F, t122254: F, t4581: F, t9079: F) -> (F, F, F, F, F) {
    let t122256 = t9704 * t24480;
    let t122258 = t34316 * t7304;
    let t122260 = t33091 * t9047;
    let t122262 = t122239 / 4.0 - 2.0 / 9.0 * t122241 + t122243 / 128.0 + t122245 / 3.0 - 2.0 / 9.0 * t122248 + t122250 / 128.0 - t122252 / 12.0 + t122254 / 9.0 + t122256 / 72.0 - t122258 / 36.0 - t122260 / 12.0;
    let t122265 = t4581 * t9079;
    (t122256, t122258, t122260, t122262, t122265)
}
