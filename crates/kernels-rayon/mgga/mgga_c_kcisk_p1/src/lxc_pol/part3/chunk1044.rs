//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1044/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1044(t227: f64, t15216: f64, t15450: f64, t218: f64, t10449: f64, t565: f64, t806: f64, t564: f64, t1629: f64, t5556: f64, t1009: f64, t3179: f64, t1053: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t15451 = t15216 + t15450;
    let t15452 = t15451 * t218;
    let t15453 = piecewise3(t228, 0.0_f64, t10449);
    let t15454 = t565 * t15453;
    let t15455 = t15454 * t806;
    let t15456 = t564 * t15455;
    let t15457 = t15456 / 16.0_f64;
    let t15458 = t1629 * t5556;
    let t15459 = t564 * t15458;
    let t15460 = 3.0_f64 / 16.0_f64 * t15459;
    let t15461 = t3179 * t1009;
    let t15462 = t15461 * t1053;
    (t15452, t15457, t15460, t15462)
}
