//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 668/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk668<F: Float>(t100: F, t2: F, t580: F, t1509: F, t2357: F, t661: F, t108: F, t105: F, t1505: F, t1507: F, t4270: F, t656: F, t662: F, t97: F) -> (F, F, F) {
    let t4273 = t100 * t2;
    let t4274 = t4273 * t580;
    let t4279 = t2357 * t1509;
    let t4280 = t4279 * t661;
    let t4283 = t108 * t2;
    let t4284 = t4283 * t580;
    let t4287 = -F::new(25.0) / F::new(9.0) * t656 * t1505 + F::new(10.0) / F::new(9.0) * t97 * t4270 + F::new(5.0) / F::new(3.0) * t97 * t4274 - F::new(25.0) / F::new(9.0) * t1507 * t662 + F::new(10.0) / F::new(9.0) * t105 * t4280 - F::new(5.0) / F::new(3.0) * t105 * t4284;
    (t4280, t4284, t4287)
}
