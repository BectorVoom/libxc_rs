//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1223/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1223(t2314: f64, t32677: f64, t4034: f64, t112594: f64, t113: f64, t119685: f64, t119792: f64, t119795: f64, t119796: f64, t119799: f64, t119810: f64, t119811: f64, t119815: f64, t119820: f64, t1393: f64, t1459: f64, t1849: f64, t1869: f64, t24980: f64, t25958: f64, t31224: f64, t31240: f64, t33080: f64, t33155: f64, t4037: f64, t650: f64, t6517: f64, t652: f64, t671: f64, t672: f64) -> f64 {
    let t119824 = 2.0_f64 * t2314 * t32677;
    let t119826 = 2.0_f64 * t4034 * t32677;
    let t119827 = -t113 * (t119685 + t119792) + t119795 - t119796 + t31240 * t1849 + t33155 * t1393 - 6.0_f64 * t119799 - 2.0_f64 * t652 * t33080 * t671 - 2.0_f64 * t31224 * t4037 - t650 * t33080 - 4.0_f64 * t6517 * t24980 - t119810 - 4.0_f64 * t119811 - 2.0_f64 * t1869 * t25958 - 2.0_f64 * t119815 * t672 - 2.0_f64 * t112594 * t1459 - 2.0_f64 * t119820 * t1459 - t119824 - t119826;
    t119827
}
