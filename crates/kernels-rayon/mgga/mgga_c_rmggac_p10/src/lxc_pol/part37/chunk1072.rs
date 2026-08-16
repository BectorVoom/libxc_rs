//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1072/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1072(t75074: f64, t75062: f64, t75065: f64, t75069: f64, t75072: f64, t75081: f64, t75092: f64, t75100: f64, t75103: f64, t75106: f64, t75108: f64, t75110: f64, t77445: f64, t77447: f64, t77450: f64, t77452: f64, t77458: f64) -> f64 {
    let t80214 = 0.65053455985619242964e-5_f64 * t75074;
    let t80221 = -0.40878380883436523435e-5_f64 * t75062 + 0.40878380883436523435e-5_f64 * t75065 + t75069 - t75072 + t80214 + t77445 - 0.31062809106223861414e-2_f64 * t75081 - t77447 - t77450 - 0.87596530464506835936e-6_f64 * t75092 + t77452 - 0.58171619854173713844e-5_f64 * t75100 - 0.72714524817717142305e-5_f64 * t75103 - 0.10511583655740820312e-5_f64 * t75106 - 0.58171619854173713844e-5_f64 * t75108 - t77458 + t75110;
    t80221
}
