//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 253/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk253(t1106: f64, t721: f64, t920: f64, t924: f64, t612: f64, t616: f64, t1076: f64, t1095: f64, t1100: f64, t1101: f64, t626: f64, t636: f64, t709: f64, t713: f64, t894: f64, t899: f64, t906: f64, t910: f64, t914: f64, t98: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1108 = t1106 * t721 / 6.0_f64;
    let t1114 = 0.01233429741534199_f64 * t920;
    let t1115 = 0.14975624337724558_f64 * t924;
    let t1116 = 0.10237773105191754_f64 * t612;
    let t1117 = 0.06825182070127836_f64 * t616;
    let t1120 = t1076 * t713 / 6.0_f64 + t1076 * t709 / 6.0_f64 - t1095 * t98 / 6.0_f64 - t1100 - t1101 * t713 / 6.0_f64 - t1101 * t709 / 6.0_f64 + t1108 + 0.01233429741534199_f64 * t894 - 0.01233429741534199_f64 * t899 - 0.01233429741534199_f64 * t906 - 0.14975624337724558_f64 * t910 - 0.14975624337724558_f64 * t914 - t1114 - t1115 - t1116 - t1117 - 0.10237773105191754_f64 * t626 - 0.10237773105191754_f64 * t636;
    (t1108, t1114, t1115, t1116, t1117, t1120)
}
