//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1188/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1188(t24165: f64, t532: f64, t12030: f64, t2092: f64, t24088: f64, t24092: f64, t3758: f64, t39913: f64, t7214: f64, t80678: f64, t80683: f64, t80687: f64, t80689: f64, t80709: f64, t80711: f64, t80714: f64) -> (f64, f64) {
    let t84347 = t532 * t24165;
    let t84389 = 0.29608813203268075857e0_f64 * t80678 - 0.14804406601634037928e0_f64 * t80683 - 0.49348022005446793095e-1_f64 * t80687 + 0.11514538467937585055e0_f64 * t80689 - 3.0_f64 * t12030 * t7214 - 18.0_f64 * t3758 * t24092 - 0.49348022005446793095e-1_f64 * t80709 - 0.15626873635058151147e0_f64 * t80711 - 0.9869604401089358619e-1_f64 * t80714 + 6.0_f64 * t3758 * t24088 - 3.0_f64 * t39913 * t2092;
    (t84347, t84389)
}
