//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1011/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1011(t1173: f64, t30605: f64, t5690: f64, t7764: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t25696: f64, t25699: f64, t25701: f64, t30569: f64, t30572: f64, t30582: f64, t30585: f64, t30606: f64) -> (f64, f64, f64) {
    let t30608 = t1173 * t30605;
    let t30610 = t5690 * t7764;
    let t30612 = -0.59793333333333333333e0_f64 * t30569 + 0.17938e1_f64 * t30572 - 0.39862222222222222223e0_f64 * t19100 + 0.19931111111111111111e0_f64 * t25590 - 0.59793333333333333333e0_f64 * t25601 + 0.29896666666666666667e0_f64 * t25609 - 0.32862666666666666666e0_f64 * t25696 + 0.16431333333333333333e0_f64 * t25699 + 0.5477111111111111111e-1_f64 * t25701 - 0.82156666666666666668e-1_f64 * t30582 + 0.49293999999999999999e0_f64 * t30585 + 0.3071625e0_f64 * t30606 + 0.1898925e1_f64 * t30608 - 0.28483875e1_f64 * t30610;
    (t30608, t30610, t30612)
}
