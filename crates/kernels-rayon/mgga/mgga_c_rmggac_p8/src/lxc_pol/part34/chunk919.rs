//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 919/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk919(t76538: f64, t1550: f64, t7778: f64, t8975: f64, t15081: f64, t68613: f64, t2416: f64, t7349: f64, t28317: f64, t3157: f64, t14387: f64, t14389: f64, t14393: f64, t14398: f64, t14399: f64, t14400: f64, t15051: f64, t15420: f64, t15423: f64, t15424: f64, t15425: f64, t15426: f64, t15427: f64, t15428: f64, t15429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76539 = 0.15965655602485078085e0_f64 * t76538;
    let t76541 = t1550 * t7778 * t8975;
    let t76542 = 0.15965655602485078085e0_f64 * t76541;
    let t76545 = t68613 * t15081;
    let t76547 = t7349 * t2416;
    let t76550 = t28317 * t3157;
    let t76586 = -t15420 + t15423 - t15424 - t15425 + t15426 + t15427 - t15428 - t15429 + t15051 + t14387 - t14389 + t14393 + t14398 - t14399 + t14400;
    (t76539, t76542, t76545, t76547, t76550, t76586)
}
