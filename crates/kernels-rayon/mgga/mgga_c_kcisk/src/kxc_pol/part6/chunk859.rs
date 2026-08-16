//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 859/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk859(t10649: f64, t15989: f64, t22564: f64, t22575: f64, t22583: f64, t28371: f64, t28375: f64, t28379: f64, t28383: f64, t28387: f64, t28391: f64, t1646: f64) -> (f64, f64) {
    let t28393 = -t10649 - 4.0_f64 / 9.0_f64 * t15989 + 2.0_f64 / 9.0_f64 * t22564 - 2.0_f64 / 3.0_f64 * t22575 + t22583 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t28371 + 4.0_f64 / 3.0_f64 * t28375 - 2.0_f64 / 3.0_f64 * t28379 - 2.0_f64 * t28383 + 2.0_f64 * t28387 - t28391 / 3.0_f64;
    let t28394 = t1646 * t28393;
    (t28393, t28394)
}
