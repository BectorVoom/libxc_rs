//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 966/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk966<F: Float>(t4279: F, t661: F, t108: F, t2: F, t580: F, t105: F, t1505: F, t1507: F, t4270: F, t4274: F, t656: F, t662: F, t97: F) -> (F, F) {
    let t4280 = t4279 * t661;
    let t4283 = t108 * t2;
    let t4284 = t4283 * t580;
    let t4287 = -F::new(25.0) / F::new(9.0) * t656 * t1505 + F::new(10.0) / F::new(9.0) * t97 * t4270 + F::new(5.0) / F::new(3.0) * t97 * t4274 - F::new(25.0) / F::new(9.0) * t1507 * t662 + F::new(10.0) / F::new(9.0) * t105 * t4280 - F::new(5.0) / F::new(3.0) * t105 * t4284;
    (t4283, t4287)
}
