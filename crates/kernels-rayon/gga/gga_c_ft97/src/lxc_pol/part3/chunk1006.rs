//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1006/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1006(t19571: f64, t684: f64, t2881: f64, t312: f64, t5299: f64, t2874: f64, t15195: f64, t4151: f64, t10514: f64, t15271: f64, t15273: f64, t1901: f64, t19535: f64, t19539: f64, t19543: f64, t19547: f64, t19551: f64, t19555: f64, t19559: f64, t19565: f64, t19568: f64, t446: f64) -> f64 {
    let t19572 = t19571 * t684;
    let t19573 = t2881 * t19572;
    let t19576 = t312 * t5299;
    let t19577 = t19576 * t684;
    let t19578 = t2874 * t19577;
    let t19581 = t15195 * t4151;
    let t19584 = 2.0_f64 / 9.0_f64 * t1901 * t19535 - 2.0_f64 / 27.0_f64 * t1901 * t19539 + 2.0_f64 / 27.0_f64 * t1901 * t19543 + 2.0_f64 / 27.0_f64 * t1901 * t19547 + t15271 + t15273 - 2.0_f64 / 3.0_f64 * t446 * t19551 - t446 * t19555 / 3.0_f64 - t446 * t19559 / 3.0_f64 + 4.0_f64 / 27.0_f64 * t10514 + 2.0_f64 / 9.0_f64 * t1901 * t19565 + 2.0_f64 / 9.0_f64 * t1901 * t19568 + t1901 * t19573 / 9.0_f64 + t1901 * t19578 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t19581;
    t19584
}
