//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1378/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1378<F: Float>(t3805: F, t9824: F, t19716: F, t415: F, t9469: F, t33515: F, t9442: F, t3959: F, t399: F, t2168: F, t3924: F, t32070: F, t20233: F, t32087: F, t33409: F, t109875: F, t110524: F, t110615: F, t113788: F, t113821: F, t113973: F, t13504: F, t2718: F, t32008: F, t33410: F, t33422: F, t33481: F, t3491: F, t3575: F, t9446: F) -> (F, F, F) {
    let t114368 = t3805 * t9824;
    let t114371 = t415 * t9469 * t19716;
    let t114377 = 0.69444444444444444446e-2 * t33515 * t9442;
    let t114378 = t399 * t3959;
    let t114379 = t3924 * t2168;
    let t114381 = t114378 * t114379 * t32070;
    let t114395 = t32087 * t20233 * t33409;
    let t114403 = -0.3684876543209876543e-3 * t114368 - 0.24872916666666666666e-2 * t114371 + 0.55555555555555555558e-1 * t3491 * t33481 * t2718 - t114377 + 0.41666666666666666668e-1 * t32087 * t114381 + 0.16083333333333333334e-1 * t32008 * t114381 + 0.46296296296296296297e-2 * t32087 * t13504 * t33422 * t3575 - 0.69444444444444444446e-2 * t32087 * t113821 - 0.37037037037037037038e-1 * t110524 * t33410 + 0.46296296296296296297e-2 * t114395 - 0.41666666666666666668e-1 * t9446 * t113788 + 0.23148148148148148148e-2 * t110615 - 0.58958024691358024689e-2 * t109875 + 0.24125e-1 * t32008 * t113973;
    (t114368, t114371, t114403)
}
