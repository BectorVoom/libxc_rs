//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1289/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1289(t7978: f64, t99059: f64, t18171: f64, t27583: f64, t28759: f64, t1307: f64, t17730: f64, t6159: f64, t18175: f64, t28766: f64, t16685: f64, t27607: f64, t28765: f64, t28772: f64, t28807: f64, t6151: f64, t94928: f64, t98171: f64, t98179: f64, t98188: f64, t99098: f64) -> (f64, f64) {
    let t99100 = 0.46336805555555555556e-3_f64 * t7978 * t99059;
    let t99108 = 0.15445601851851851852e-3_f64 * t27583 * t18171 * t28759;
    let t99110 = t6159 * t17730 * t1307;
    let t99117 = 0.10297067901234567901e-3_f64 * t27583 * t18175 * t28766;
    let t99118 = -0.38691203703703703703e-3_f64 * t98171 + 0.38691203703703703703e-3_f64 * t98179 + 0.69505208333333333334e-3_f64 * t27607 * t28772 - t99098 - t99100 + 0.19345601851851851852e-2_f64 * t98188 - 0.15445601851851851852e-3_f64 * t27583 * t6151 * t28765 * t16685 + t99108 + 0.23168402777777777778e-3_f64 * t27583 * t99110 + 0.23168402777777777778e-3_f64 * t94928 * t28807 - t99117;
    (t99110, t99118)
}
