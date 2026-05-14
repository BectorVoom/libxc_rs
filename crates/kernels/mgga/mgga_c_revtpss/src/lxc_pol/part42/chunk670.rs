//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 670/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk670<F: Float>(t1310: F, t1518: F, t1514: F, t625: F, t1513: F, t2339: F, t665: F, t1504: F, t2349: F, t658: F, t100: F, t2: F, t580: F, t1509: F, t2357: F, t661: F) -> (F, F, F, F, F, F, F) {
    let t4257 = t1310 * t1518;
    let t4261 = t625 * t1514;
    let t4263 = t2339 * t1513;
    let t4264 = t4263 * t665;
    let t4269 = t2349 * t1504;
    let t4270 = t4269 * t658;
    let t4273 = t100 * t2;
    let t4274 = t4273 * t580;
    let t4279 = t2357 * t1509;
    let t4280 = t4279 * t661;
    (t4257, t4261, t4263, t4264, t4270, t4274, t4280)
}
