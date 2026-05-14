//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1401/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1401<F: Float>(t14294: F, t4321: F, t9831: F, t113348: F, t113351: F, t113353: F, t113356: F, t113359: F, t113362: F, t113365: F, t113367: F, t113370: F, t113373: F, t113376: F, t113379: F, t113381: F, t113383: F, t113385: F, t113387: F, t113389: F, t113391: F, t113393: F, t113395: F) -> (F, F) {
    let t114866 = 6.0 * t14294 * t9831 * t4321;
    let t114887 = 0.89930555555555555557e-2 * t113348 + 0.53958333333333333334e-1 * t113351 + 0.26979166666666666667e-1 * t113353 - 0.9375e-1 * t113356 + 0.95925925925925925927e-1 * t113359 - 0.809375e-1 * t113362 + 0.12140625e0 * t113365 - 0.4046875e-1 * t113367 - 0.33333333333333333334e0 * t113370 - 0.17986111111111111111e-1 * t113373 + 0.125e0 * t113376 + 0.25e0 * t113379 + 0.625e-1 * t113381 - 0.26979166666666666667e-1 * t113383 + 0.1875e0 * t113385 + 0.11111111111111111111e0 * t113387 - 0.20833333333333333333e-1 * t113389 - 0.125e0 * t113391 - 0.125e0 * t113393 - 0.21583333333333333334e0 * t113395;
    (t114866, t114887)
}
