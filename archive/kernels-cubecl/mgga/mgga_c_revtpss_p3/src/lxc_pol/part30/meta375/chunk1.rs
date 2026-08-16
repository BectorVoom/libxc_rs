//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1414/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1414<F: Float>(t13458: F, t665: F, t2366: F, t4263: F, t10227: F, t1504: F, t2350: F, t2349: F, t97: F, t2255: F, t658: F, t2256: F, t4269: F) -> (F, F, F, F, F, F) {
    let t13459 = t13458 * t665;
    let t13462 = t4263 * t2366;
    let t13472 = t10227 * t1504 * t2350;
    let t13475 = t97 * t2349;
    let t13476 = t2255 * t658;
    let t13479 = t4269 * t2256;
    (t13459, t13462, t13472, t13475, t13476, t13479)
}
