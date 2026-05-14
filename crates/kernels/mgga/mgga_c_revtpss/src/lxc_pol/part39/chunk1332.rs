//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1332/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1332<F: Float>(t1513: F, t2340: F, t4287: F, t665: F, t2366: F, t13514: F, t93: F, t31087: F, t575: F, t1464: F, t8283: F, t10208: F, t625: F, t31036: F, t31027: F, t31040: F) -> (F, F, F, F, F, F, F, F, F) {
    let t101457 = t1513 * t2340;
    let t101460 = t4287 * t665;
    let t101463 = t1513 * t2366;
    let t101522 = t93 * t13514;
    let t116890 = t31087 * t575;
    let t116899 = t8283 * t1464;
    let t116912 = t625 * t10208;
    let t116913 = t116912 * t31036;
    let t116915 = t31027 * t31040;
    (t101457, t101460, t101463, t101522, t116890, t116899, t116912, t116913, t116915)
}
