//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 514/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk514<F: Float>(t114: F, t107: F, t661: F, t2256: F, t108: F, t101: F, t105: F, t2344: F, t2351: F, t2354: F, t656: F, t659: F, t97: F, t655: F, t2335: F, t2336: F, t2341: F, t69: F) -> (F, F, F, F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t2357 = 1.0 / t107;
    let t2358 = t661 * t661;
    let t2359 = t2357 * t2358;
    let t2362 = -t2256;
    let t2363 = t108 * t2362;
    let t2366 = 40.0 / 9.0 * t2344 * t101 - 50.0 / 9.0 * t656 * t659 + 10.0 / 9.0 * t97 * t2351 + 5.0 / 3.0 * t97 * t2354 + 10.0 / 9.0 * t105 * t2359 + 5.0 / 3.0 * t105 * t2363;
    let t2367 = t655 * t2366;
    let t2371 = piecewise3(t115, 0.0, t2335 + 2.0 / 3.0 * t2336 + t69 * t2341 / 4.0 - t69 * t2367 / 8.0);
    (t2357, t2358, t2359, t2362, t2363, t2366, t2367, t2371)
}
