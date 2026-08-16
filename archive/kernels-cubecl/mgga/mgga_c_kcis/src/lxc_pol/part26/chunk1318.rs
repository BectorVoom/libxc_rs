//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1318/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1318<F: Float>(t102580: F, t6151: F, t94960: F, t21850: F, t4160: F, t94425: F, t102568: F, t102582: F, t18079: F, t20984: F, t21655: F, t27567: F, t27583: F, t28755: F, t28758: F, t28767: F, t29583: F, t94928: F, t98719: F, t99301: F, t99446: F, t99565: F, t99578: F) -> (F, F) {
    let t102602 = t6151 * t94960 * t102580;
    let t102621 = t4160 * t94425 * t21850;
    let t102623 = -F::cast_from(0.30918233506944444445e-4_f64) * t27567 * t102582 - F::cast_from(0.92673611111111111112e-3_f64) * t27583 * t18079 * t28758 * t21655 + F::cast_from(0.15445601851851851852e-3_f64) * t27583 * t102602 + F::cast_from(0.23168402777777777778e-3_f64) * t94928 * t29583 + F::cast_from(0.92673611111111111112e-3_f64) * t27583 * t6151 * t99446 * t20984 + F::cast_from(0.20612155671296296296e-4_f64) * t27567 * t102602 - F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t102568 - F::cast_from(0.77382407407407407407e-3_f64) * t98719 + F::cast_from(0.30918233506944444445e-4_f64) * t99565 * t28755 - F::cast_from(0.30891203703703703704e-3_f64) * t99301 * t28767 + t99578 - F::cast_from(0.15476481481481481481e-2_f64) * t102621;
    (t102621, t102623)
}
