//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 652/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk652<F: Float>(t661: F, t2357: F, t2256: F, t108: F, t101: F, t105: F, t2344: F, t2351: F, t2354: F, t656: F, t659: F, t97: F) -> (F, F, F, F, F) {
    let t2358 = t661 * t661;
    let t2359 = t2357 * t2358;
    let t2362 = -t2256;
    let t2363 = t108 * t2362;
    let t2366 = F::new(40.0) / F::new(9.0) * t2344 * t101 - F::new(50.0) / F::new(9.0) * t656 * t659 + F::new(10.0) / F::new(9.0) * t97 * t2351 + F::new(5.0) / F::new(3.0) * t97 * t2354 + F::new(10.0) / F::new(9.0) * t105 * t2359 + F::new(5.0) / F::new(3.0) * t105 * t2363;
    (t2358, t2359, t2362, t2363, t2366)
}
