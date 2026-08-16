//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1103/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1103(t38311: f64, t38322: f64, t38336: f64, t38341: f64, t38346: f64, t38349: f64, t38362: f64, t11554: f64, t2262: f64, t6897: f64, t910: f64, t2330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39121 = 0.13010691197123848592e-3_f64 * t38311;
    let t39122 = 0.26021382394247697185e-3_f64 * t38322;
    let t39127 = 0.2439011983326002265e-2_f64 * t38336;
    let t39129 = 0.2439011983326002265e-2_f64 * t38341;
    let t39130 = 0.18292589874945016987e-2_f64 * t38346;
    let t39131 = 0.30487649791575028312e-3_f64 * t38349;
    let t39134 = 0.91462949374725084936e-3_f64 * t38362;
    let t39178 = t11554 * t2262;
    let t39182 = t6897 * t910;
    let t39183 = t39182 * t2330;
    (t39121, t39122, t39127, t39129, t39130, t39131, t39134, t39178, t39183)
}
