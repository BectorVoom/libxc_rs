//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3489/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3489(t15772: f64, t4834: f64, t1042: f64, t1045: f64, t15830: f64, t15850: f64, t18281: f64, t19622: f64, t19675: f64, t19682: f64, t2858: f64, t3059: f64, t3075: f64, t3106: f64, t3117: f64, t3127: f64, t42121: f64, t42124: f64, t42141: f64, t43291: f64, t43297: f64, t4803: f64, t4872: f64, t53389: f64, t5825: f64, t6271: f64, t999: f64) -> f64 {
    let t65689 = t4834 * t15772;
    let t65693 = -0.28582678745379824648e-3_f64 * t3127 * t1042 * t4872 * t18281 * t999 - 0.14291339372689912324e-3_f64 * t3127 * t1042 * t4872 * t5825 * t3075 - t42121 - 0.47637797908966374413e-4_f64 * t42124 - 0.16090989515917530913e-2_f64 * t42141 - 0.25724410870841842183e-2_f64 * t43291 * t3117 * t6271 * t1045 * t3059 - 0.91464571985215438873e-2_f64 * t43297 * t19622 + 0.19055119163586549765e-3_f64 * t53389 + 0.28582678745379824648e-3_f64 * t3127 * t1042 * t19675 * t2858 + 0.30488190661738479624e-2_f64 * t3106 * t19682 + 0.60976381323476959249e-2_f64 * t15830 * t4803 - 0.76220476654346199061e-3_f64 * t65689 - 0.11433071498151929859e-2_f64 * t15850 * t4803;
    t65693
}
