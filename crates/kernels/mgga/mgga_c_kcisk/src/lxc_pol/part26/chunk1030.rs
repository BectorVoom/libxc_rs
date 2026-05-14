//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1030/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1030<F: Float>(t27426: F, t4209: F, t27220: F, t27223: F, t27225: F, t27228: F, t27231: F, t27233: F, t27237: F, t27398: F, t27400: F, t27404: F, t27407: F, t27409: F, t27412: F, t27414: F, t27417: F, t27419: F, t27421: F, t27424: F) -> (F, F) {
    let t27427 = t4209 * t27426;
    let t27429 = -t27220 / 9.0 - t27223 / 48.0 - 2.0 / 9.0 * t27225 - t27228 / 24.0 - t27231 / 128.0 + t27233 / 8.0 + t27237 / 36.0 + t27398 / 16.0 + t27400 / 24.0 - t27404 / 256.0 + t27407 / 108.0 + t27409 / 18.0 - t27412 / 48.0 + t27414 / 128.0 + t27417 / 24.0 + t27419 / 48.0 + t27421 / 12.0 + t27424 / 288.0 - t27427 / 3.0;
    (t27427, t27429)
}
