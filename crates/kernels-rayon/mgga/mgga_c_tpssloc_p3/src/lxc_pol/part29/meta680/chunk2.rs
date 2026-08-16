//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2288/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2288(t24574: f64, t27427: f64, t5052: f64, t7284: f64, t14980: f64, t15803: f64, t1761: f64, t2155: f64, t24868: f64, t27382: f64, t27742: f64, t3477: f64, t3593: f64, t4945: f64, t5055: f64, t51928: f64, t7283: f64, t7287: f64, t7351: f64, t7356: f64, t7392: f64, t86400: f64, t86409: f64, t86424: f64) -> f64 {
    let t94676 = 0.18277045187202515961e-2_f64 * t24574 * t27427;
    let t94680 = t7284 * t5052;
    let t94698 = -t94676 + 0.82246703342411321825e-2_f64 * t7283 * t3477 * t27382 - 0.54831135561607547884e-2_f64 * t7283 * t94680 * t7287 - t5055 * t24868 + 0.12184696791468343974e-2_f64 * t86409 - 2.0_f64 * t14980 * t7392 - t86400 * t1761 - t4945 * t24868 + 2.0_f64 * t7351 * t15803 - 2.0_f64 * t3593 * t27742 - 0.27415567780803773942e-2_f64 * t86424 + 4.0_f64 * t14980 * t7356 - t51928 * t2155;
    t94698
}
