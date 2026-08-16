//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1318/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1318(t102580: f64, t6151: f64, t94960: f64, t21850: f64, t4160: f64, t94425: f64, t102568: f64, t102582: f64, t18079: f64, t20984: f64, t21655: f64, t27567: f64, t27583: f64, t28755: f64, t28758: f64, t28767: f64, t29583: f64, t94928: f64, t98719: f64, t99301: f64, t99446: f64, t99565: f64, t99578: f64) -> (f64, f64) {
    let t102602 = t6151 * t94960 * t102580;
    let t102621 = t4160 * t94425 * t21850;
    let t102623 = -0.30918233506944444445e-4_f64 * t27567 * t102582 - 0.92673611111111111112e-3_f64 * t27583 * t18079 * t28758 * t21655 + 0.15445601851851851852e-3_f64 * t27583 * t102602 + 0.23168402777777777778e-3_f64 * t94928 * t29583 + 0.92673611111111111112e-3_f64 * t27583 * t6151 * t99446 * t20984 + 0.20612155671296296296e-4_f64 * t27567 * t102602 - 0.23168402777777777778e-3_f64 * t27583 * t102568 - 0.77382407407407407407e-3_f64 * t98719 + 0.30918233506944444445e-4_f64 * t99565 * t28755 - 0.30891203703703703704e-3_f64 * t99301 * t28767 + t99578 - 0.15476481481481481481e-2_f64 * t102621;
    (t102621, t102623)
}
