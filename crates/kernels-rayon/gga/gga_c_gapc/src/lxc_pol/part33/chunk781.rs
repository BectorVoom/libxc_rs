//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 781/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk781(t3017: f64, t5022: f64, t1043: f64, t3157: f64, t8948: f64, t1645: f64, t190: f64, t1649: f64, t1643: f64, t9135: f64, t9138: f64, t9140: f64, t9142: f64, t9145: f64, t9148: f64, t9151: f64, t9153: f64, t9156: f64, t9158: f64) -> (f64, f64, f64) {
    let pi = (M_PI as f64);
    let t9160 = t3017 * t5022;
    let t9161 = t1043 * t9160;
    let t9163 = t8948 * t3157;
    let t9166 = t190 * t1645 * pi;
    let t9167 = t9166 * t1649;
    let t9168 = t1643 * t9167;
    let t9170 = 0.13900948042322754167e-2_f64 * t9135 + 0.10120768229166666667e-4_f64 * t9138 - 0.6487109086417285278e-2_f64 * t9140 + 0.1374296967252737644e-5_f64 * t9142 - 0.38647271295071362318e-6_f64 * t9145 + 0.687148483626368822e-6_f64 * t9148 - 0.21135226489492151266e-6_f64 * t9151 + 0.42270452978984302532e-6_f64 * t9153 + 0.27801896084645508334e-2_f64 * t9156 + 0.33816362383187442026e-4_f64 * t9158 + 0.43478180206955282604e-5_f64 * t9161 - 0.19679271556712962963e-4_f64 * t9163 + 0.38010404803226280926e-3_f64 * t9168;
    (t9160, t9166, t9170)
}
