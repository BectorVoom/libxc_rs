//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3269/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3269(t10811: f64, t18647: f64, t18511: f64, t40864: f64, t10905: f64, t18515: f64, t10744: f64, t18409: f64, t808: f64, t18414: f64, t40521: f64, t10900: f64, t14468: f64, t1548: f64, t18393: f64, t18444: f64, t2430: f64, t2724: f64, t2730: f64, t4362: f64, t4364: f64, t50968: f64, t50974: f64, t5984: f64, t5988: f64, t775: f64, t800: f64) -> f64 {
    let t62045 = t10811 * t18647;
    let t62056 = t40864 * t18511;
    let t62058 = t10905 * t18515;
    let t62069 = t10744 * t808 * t18409;
    let t62072 = t40521 * t808 * t18414;
    let t62074 = 0.20007875121765877254e-2_f64 * t50968 + 0.12862205435420921092e-2_f64 * t4362 * t4364 * t18444 * t2724 - 0.16006300097412701803e-1_f64 * t62045 + 0.80031500487063509016e-1_f64 * t50974 + t2730 * t800 * t18393 * t775 / 8.0_f64 + t2730 * t800 * t5984 * t2430 / 16.0_f64 + 7.0_f64 / 6.0_f64 * t62056 - 7.0_f64 / 12.0_f64 * t62058 - t10900 * t800 * t5988 * t2430 / 4.0_f64 + t2730 * t800 * t1548 * t14468 / 8.0_f64 + 0.25410001404642664112e-5_f64 * t62069 - 0.50820002809285328225e-5_f64 * t62072;
    t62074
}
