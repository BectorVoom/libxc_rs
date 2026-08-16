//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2353/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2353(t1388: f64, t25988: f64, t22574: f64, t26162: f64, t26149: f64, t6876: f64, t19577: f64, t31035: f64, t12557: f64, t1266: f64, t15857: f64, t1869: f64, t2363: f64, t26098: f64, t5361: f64, t6517: f64, t652: f64, t672: f64, t6872: f64, t7670: f64, t90400: f64, t90428: f64, t90434: f64, t90436: f64, t90440: f64, t90444: f64, t90447: f64, t90450: f64, t90454: f64, t90456: f64, t91564: f64) -> f64 {
    let t91565 = t25988 * t1388;
    let t91568 = 12.0_f64 * t22574 * t26162 * t91565;
    let t91570 = 2.0_f64 * t6876 * t26149;
    let t91573 = 6.0_f64 * t22574 * t31035 * t19577;
    let t91574 = -2.0_f64 * t2363 * t652 * t7670 - 2.0_f64 * t12557 * t6517 - 2.0_f64 * t1266 * t26098 - t15857 * t1869 + 2.0_f64 * t5361 * t6872 - 4.0_f64 * t672 * t90400 - t90428 + t90434 - t90436 + t90440 + t90444 + t90447 - t90450 - t90454 - t90456 + t91564 + t91568 - t91570 - t91573;
    t91574
}
