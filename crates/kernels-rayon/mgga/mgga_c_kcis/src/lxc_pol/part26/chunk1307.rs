//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1307/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1307(t1498: f64, t303: f64, t7194: f64, t27475: f64, t7258: f64, t20994: f64, t4160: f64, t98661: f64, t102221: f64, t18183: f64, t18256: f64, t20882: f64, t20984: f64, t2260: f64, t27583: f64, t28765: f64, t6151: f64, t8217: f64, t94472: f64, t94537: f64, t94539: f64, t94966: f64, t99120: f64, t99392: f64, t99411: f64) -> (f64, f64, f64, f64) {
    let t102381 = t303 * t7194 * t1498;
    let t102384 = t303 * t27475 * t7258;
    let t102398 = t4160 * t98661 * t20994;
    let t102400 = -0.25794135802469135802e-3_f64 * t94472 + t99392 - 0.15445601851851851852e-3_f64 * t27583 * t6151 * t28765 * t20882 + 0.11607361111111111111e-2_f64 * t102381 - 0.17411041666666666666e-2_f64 * t102384 + t99411 + 0.18534722222222222222e-2_f64 * t18256 * t8217 * t2260 + 0.38691203703703703703e-3_f64 * t94537 - 0.25794135802469135802e-3_f64 * t94539 - 0.30945286961263020834e-5_f64 * t94966 * t102221 - 0.36039737654320987655e-3_f64 * t27583 * t18183 * t99120 * t20984 + 0.46429444444444444444e-2_f64 * t102398;
    (t102381, t102384, t102398, t102400)
}
