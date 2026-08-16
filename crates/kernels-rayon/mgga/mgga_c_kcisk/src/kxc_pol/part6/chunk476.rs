//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 476/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk476(t3529: f64, t41: f64, t3532: f64, t451: f64, t1390: f64, t470: f64, t1555: f64, t547: f64, t524: f64, t544: f64, t1587: f64, t538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4271 = t41 * t3529;
    let t4272 = t451 * t3532;
    let t4282 = t451 * t1390;
    let t4304 = 1.0_f64 / t470;
    let t4346 = 1.0_f64 / t1555 / t547;
    let t4347 = t524 * t4346;
    let t4349 = t544 * t544;
    let t4350 = 1.0_f64 / t4349;
    let t4374 = 1.0_f64 / t1587 / t538;
    (t4271, t4272, t4282, t4304, t4346, t4347, t4349, t4350, t4374)
}
