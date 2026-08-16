//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 894/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk894(t21126: f64, t908: f64, t136: f64, t21122: f64, t2826: f64, t10577: f64, t13598: f64, t17149: f64, t17165: f64, t17175: f64, t21124: f64, t21128: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> (f64, f64, f64) {
    let t21160 = t908 * t21126;
    let t21161 = t136 * t21160;
    let t21167 = t2826 * t21122;
    let t21168 = t136 * t21167;
    let t21180 = -t10577 - 4.0_f64 / 9.0_f64 * t13598 + 2.0_f64 / 9.0_f64 * t17149 - 2.0_f64 / 3.0_f64 * t17165 + t17175 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t21147 + 4.0_f64 / 3.0_f64 * t21150 - 2.0_f64 / 3.0_f64 * t21124 - 2.0_f64 * t21153 + 2.0_f64 * t21128 - t21156 / 3.0_f64;
    (t21161, t21168, t21180)
}
