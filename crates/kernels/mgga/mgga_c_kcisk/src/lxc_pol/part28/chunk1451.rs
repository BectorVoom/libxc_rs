//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1451/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1451<F: Float>(t122156: F, t122158: F, t122160: F, t122163: F, t122166: F, t122169: F, t122365: F, t122367: F, t122369: F, t122372: F, t122374: F, t122375: F, t122376: F, t122379: F, t122380: F, t122382: F, t122403: F, t123328: F, t123443: F, t123464: F, t123465: F, t123466: F, t240: F) -> (F,) {
    let t123470 = t122156 - t122158 + t122160 + t122163 + t122166 - t122169 - t122365 + t122367 - t122369 + t122372 - t122374 - t122375 - t122376 - t122379 - t122380 - t122382 + t240 * (t122403 + t123328 + t123443 + t123466) + t123464 - t123465;
    (t123470,)
}
