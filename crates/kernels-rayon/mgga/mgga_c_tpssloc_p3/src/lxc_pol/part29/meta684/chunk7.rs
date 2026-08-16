//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2333/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2333(t24574: f64, t27392: f64, t1170: f64, t2121: f64, t27766: f64, t1238: f64, t15794: f64, t1716: f64, t24567: f64, t24568: f64, t24582: f64, t24630: f64, t24639: f64, t24877: f64, t24893: f64, t27406: f64, t27415: f64, t3598: f64, t3630: f64, t4945: f64, t5055: f64, t5060: f64, t7283: f64, t7351: f64, t8087: f64, t86473: f64, t86494: f64) -> f64 {
    let t95863 = 0.54831135561607547884e-2_f64 * t24574 * t27392;
    let t95866 = 0.54831135561607547884e-2_f64 * t2121 * t1170 * t27766;
    let t95876 = -6.0_f64 * t7351 * t15794 + 0.43864908449286038306e-1_f64 * t27406 * t24568 + 0.43864908449286038306e-1_f64 * t27406 * t24630 + 4.0_f64 * t4945 * t24582 - 0.43864908449286038306e-1_f64 * t27406 * t24639 + 4.0_f64 * t24893 * t5060 - 0.16449340668482264365e-1_f64 * t7283 * t24567 * t27415 + 0.12184696791468343974e-2_f64 * t86473 + t95863 + t95866 + 0.16449340668482264365e-1_f64 * t7283 * t1716 * t86494 + 2.0_f64 * t1238 * t3598 * t8087 * t3630 + 2.0_f64 * t5055 * t24877;
    t95876
}
