//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 871/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk871(t28312: f64, t682: f64, t2372: f64, t8522: f64, t11371: f64, t15989: f64, t22564: f64, t22575: f64, t22583: f64, t28371: f64, t28375: f64, t28379: f64, t28383: f64, t28387: f64, t28391: f64, t28394: f64, t28412: f64, t28417: f64) -> (f64, f64, f64) {
    let t28539 = t682 * t28312;
    let t28546 = t2372 * t8522;
    let t28568 = 0.14865e-1_f64 * t28417 - 0.2973e-1_f64 * t28412 + 0.1982e-1_f64 * t28394 - t11371 - 0.55033333333333333332e-2_f64 * t15989 + 0.27516666666666666666e-2_f64 * t22564 - 0.82549999999999999999e-2_f64 * t22575 + 0.41274999999999999999e-2_f64 * t22583 - 0.45861111111111111112e-2_f64 * t28371 + 0.1651e-1_f64 * t28375 - 0.82550000000000000001e-2_f64 * t28379 - 0.24765e-1_f64 * t28383 + 0.24765e-1_f64 * t28387 - 0.41275e-2_f64 * t28391;
    (t28539, t28546, t28568)
}
