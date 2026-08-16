//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1172/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1172(t1029: f64, t160: f64, t1634: f64, t1692: f64, t1747: f64, t1750: f64, t1773: f64, t19867: f64, t20397: f64, t2575: f64, t2625: f64, t2631: f64, t5357: f64, t5361: f64, t568: f64, t596: f64, t614: f64, t6853: f64, t7065: f64, t7074: f64, t7075: f64, t7078: f64, t8865: f64) -> f64 {
    let t20398 = 180.0_f64 * t1634 * t1773 * t2575 * t2631 - 36.0_f64 * t2631 * t568 * t614 * t6853 + 3.0_f64 * t160 * t19867 * t596 - 36.0_f64 * t1692 * t2631 * t7074 + 60.0_f64 * t1029 * t5357 - 36.0_f64 * t1747 * t2625 + 9.0_f64 * t1750 * t2625 - 36.0_f64 * t5361 * t8865 - 72.0_f64 * t7065 * t7075 - 36.0_f64 * t7065 * t7078 + t20397;
    t20398
}
