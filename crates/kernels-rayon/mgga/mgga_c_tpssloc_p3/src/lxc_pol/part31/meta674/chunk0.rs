//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2036/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2036(t102386: f64, t1266: f64, t1393: f64, t19461: f64, t2040: f64, t2075: f64, t2314: f64, t24432: f64, t24995: f64, t26161: f64, t26558: f64, t26872: f64, t26878: f64, t26880: f64, t27171: f64, t28030: f64, t28943: f64, t28951: f64, t28952: f64, t29219: f64, t29241: f64, t29380: f64, t4028: f64, t4034: f64, t5457: f64, t652: f64, t672: f64, t6876: f64, t7050: f64, t7156: f64, t75210: f64, t7685: f64, t91655: f64, t96709: f64, t97902: f64, t97933: f64) -> f64 {
    let t103029 = -4.0_f64 * t4028 * t27171 - 2.0_f64 * t96709 * t2040 - 2.0_f64 * t97933 * t2040 - 2.0_f64 * t28030 * t7050 - 12.0_f64 * t24995 * t24432 * t97902 + t29241 * t1393 - 2.0_f64 * t102386 * t672 - 4.0_f64 * t2314 * t29219 + 6.0_f64 * t6876 * t29380 - 2.0_f64 * t7685 * t26880 - 2.0_f64 * t7685 * t26878 + 2.0_f64 * t26161 * t26558 * t75210 - t28943 * t1266 - 2.0_f64 * t19461 * t2075 - 2.0_f64 * t5457 * t7156 - 6.0_f64 * t91655 * t26872 - 2.0_f64 * t2314 * t28952 - 2.0_f64 * t4034 * t28952 - 2.0_f64 * t652 * t1266 * t28951;
    t103029
}
