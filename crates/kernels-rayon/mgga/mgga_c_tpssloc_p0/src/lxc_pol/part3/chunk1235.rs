//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1235/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1235(t120: f64, t5286: f64, t3805: f64, t3807: f64, t12407: f64, t5249: f64, t12284: f64, t12301: f64, t12397: f64, t12429: f64, t1341: f64, t1363: f64, t16147: f64, t16150: f64, t16155: f64, t16159: f64, t16208: f64, t16211: f64, t16214: f64, t16217: f64, t16227: f64, t16233: f64, t16235: f64, t16239: f64, t16241: f64, t1827: f64, t3778: f64, t3803: f64, t5259: f64, t5289: f64) -> (f64, f64) {
    let t16242 = t120 * t5286;
    let t16244 = t3805 * t16242 * t3807;
    let t16248 = t3805 * t5249 * t12407;
    let t16253 = -t16147 + 5.0_f64 / 384.0_f64 * t1363 * t16150 + 5.0_f64 / 768.0_f64 * t1363 * t16155 + t16159 - t1341 * t16208 / 3072.0_f64 - 119.0_f64 / 13824.0_f64 * t16211 + t16214 - 5.0_f64 / 128.0_f64 * t1363 * t16217 - t12397 * t1827 / 3072.0_f64 - t3778 * t5289 / 1536.0_f64 - 5.0_f64 / 384.0_f64 * t3803 * t16227 - 7.0_f64 / 576.0_f64 * t12284 + 7.0_f64 / 2304.0_f64 * t12301 - t16233 * t16235 / 512.0_f64 - t16239 + t16241 + t3803 * t16244 / 384.0_f64 + t3803 * t16248 / 768.0_f64 + t12429 * t5259 / 384.0_f64;
    (t16242, t16253)
}
