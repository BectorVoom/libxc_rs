//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 928/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk928(t568: f64, t7074: f64, t1692: f64, t2632: f64, t596: f64, t6853: f64, t1029: f64, t1031: f64, t160: f64, t162: f64, t1742: f64, t1747: f64, t1750: f64, t2625: f64, t2631: f64, t2633: f64, t2636: f64, t594: f64, t597: f64, t7055: f64, t7065: f64, t7071: f64) -> (f64, f64, f64, f64) {
    let t7075 = t7074 * t568;
    let t7078 = t2632 * t1692;
    let t7081 = t596 * t6853;
    let t7084 = -12.0_f64 * t1029 * t1747 + 3.0_f64 * t1029 * t1750 + 3.0_f64 * t1031 * t1742 + 3.0_f64 * t160 * t7081 - t162 * t7055 + 6.0_f64 * t2625 * t597 + 60.0_f64 * t2631 * t7071 - 24.0_f64 * t2631 * t7075 - 12.0_f64 * t2631 * t7078 - 24.0_f64 * t2633 * t7065 + 6.0_f64 * t2636 * t594;
    (t7075, t7078, t7081, t7084)
}
