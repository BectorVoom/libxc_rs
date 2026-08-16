//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 998/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk998(t10137: f64, t5373: f64, t3240: f64, t5377: f64, t1206: f64, t5372: f64, t762: f64, t1629: f64, t4397: f64, t5376: f64, t10078: f64, t10104: f64, t10141: f64, t1244: f64, t12902: f64, t13756: f64, t13760: f64, t13765: f64, t13768: f64, t13771: f64, t3244: f64, t3271: f64, t4413: f64) -> f64 {
    let t13774 = t10137 * t5373;
    let t13776 = t3240 * t5377;
    let t13780 = t762 * t5372 * t1206;
    let t13784 = t762 * t1629 * t4397;
    let t13788 = t762 * t5376 * t1206;
    let t13791 = t3271 * t13756 / 384.0_f64 - t4413 * t13760 / 384.0_f64 + t4413 * t13765 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t13768 - t1244 * t13771 / 768.0_f64 - 7.0_f64 / 48.0_f64 * t13774 + 7.0_f64 / 144.0_f64 * t13776 + t12902 - 119.0_f64 / 13824.0_f64 * t10078 - t10104 - t10141 * t13780 / 4.0_f64 + t3244 * t13784 / 8.0_f64 + t3244 * t13788 / 16.0_f64;
    t13791
}
