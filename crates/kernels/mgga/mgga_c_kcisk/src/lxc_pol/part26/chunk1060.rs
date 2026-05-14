//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1060/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1060<F: Float>(t27220: F, t27223: F, t27225: F, t27228: F, t27231: F, t27233: F, t27237: F, t27398: F, t27400: F, t27404: F, t27407: F, t27409: F, t27412: F, t27414: F, t27417: F, t27419: F, t27421: F, t27424: F, t27427: F) -> (F,) {
    let t28134 = -0.16666666666666666667e0 * t27220 - 0.53958333333333333332e-1 * t27223 - 0.33333333333333333333e0 * t27225 - 0.625e-1 * t27228 - 0.20234375e-1 * t27231 + 0.1875e0 * t27233 + 0.41666666666666666667e-1 * t27237 + 0.9375e-1 * t27398 + 0.625e-1 * t27400 - 0.101171875e-1 * t27404 + 0.23981481481481481481e-1 * t27407 + 0.14388888888888888889e0 * t27409 - 0.53958333333333333333e-1 * t27412 + 0.20234375e-1 * t27414 + 0.10791666666666666667e0 * t27417 + 0.53958333333333333333e-1 * t27419 + 0.125e0 * t27421 + 0.89930555555555555553e-2 * t27424 - 0.5e0 * t27427;
    (t28134,)
}
