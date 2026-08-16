//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1371/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1371(t20284: f64, t71: f64, t33: f64, t75284: f64, t1437: f64, t5441: f64, t72: f64, t3953: f64, t5392: f64, t1860: f64, t1863: f64, t1864: f64, t1865: f64, t20217: f64, t20234: f64, t21510: f64, t22505: f64, t26084: f64, t27949: f64, t27950: f64, t27953: f64, t27956: f64, t27957: f64, t27972: f64, t6490: f64, t6500: f64, t67: f64, t7428: f64, t7435: f64, t7441: f64, t7445: f64, t83796: f64, t83803: f64) -> f64 {
    let t106800 = t71 * t20284;
    let t106804 = t75284 * t33;
    let t106813 = t72 * t5441 * t1437;
    let t106816 = t3953 * t5392;
    let t106819 = -t7428 * t27957 / 2.0_f64 - t1860 * (-5.0_f64 / 108.0_f64 * t83796 * t20234 + 5.0_f64 / 6.0_f64 * t22505 * t21510 + 5.0_f64 / 6.0_f64 * t6500 * t20217 + t83803) * t67 * t1864 / 6.0_f64 - t1860 * t27949 * t7445 / 2.0_f64 - t1860 * t7441 * t27956 / 2.0_f64 - t1860 * t1863 * t106800 / 6.0_f64 - t106804 * t1865 / 6.0_f64 + t7435 * t27950 + 5.0_f64 * t26084 * t27972 + 2.0_f64 * t7435 * t27953 + 5.0_f64 / 2.0_f64 * t6490 * t106813 + t106816 * t1865 + t7435 * t27957;
    t106819
}
