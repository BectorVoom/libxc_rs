//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 409/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk409<F: Float>(t2029: F, t2647: F, t1994: F, t2033: F, t2475: F, t2511: F, t2530: F, t2535: F, t2539: F, t2618: F, t795: F, t2561: F, t2565: F, t2569: F, t2573: F, t2577: F, t2581: F, t2588: F, t2592: F) -> (F, F, F) {
    let t2648 = t2647 * t2029;
    let t2656 = t2618 * t795 - 0.193e0 * t1994 * t2648 + t2033 + 0.11607361111111111111e-2 * t2475 + 0.17411041666666666666e-2 * t2511 - 0.17411041666666666666e-2 * t2530 - 0.46429444444444444443e-2 * t2535 + 0.11607361111111111111e-2 * t2539;
    let t2666 = 0.9375e-1 * t2561 - 0.9375e-1 * t2565 - 0.25e0 * t2569 + 0.625e-1 * t2573 - 0.101171875e-1 * t2577 + 0.101171875e-1 * t2581 + 0.53958333333333333333e-1 * t2588 - 0.13489583333333333333e-1 * t2592;
    (t2648, t2656, t2666)
}
