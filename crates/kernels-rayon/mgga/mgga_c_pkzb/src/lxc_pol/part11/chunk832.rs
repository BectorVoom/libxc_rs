//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 832/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk832(t3396: f64, t614: f64, t568: f64, t596: f64, t8817: f64, t1029: f64, t1031: f64, t160: f64, t162: f64, t2625: f64, t2631: f64, t2633: f64, t2636: f64, t3431: f64, t3435: f64, t3438: f64, t594: f64, t597: f64, t8859: f64, t8865: f64, t8873: f64, t8876: f64) -> (f64, f64, f64) {
    let t8881 = t614 * t3396;
    let t8882 = t8881 * t568;
    let t8885 = t596 * t8817;
    let t8888 = 6.0_f64 * t1029 * t2636 + 6.0_f64 * t1031 * t2625 + 3.0_f64 * t160 * t8885 - t162 * t8859 + 60.0_f64 * t2631 * t8873 - 24.0_f64 * t2631 * t8876 - 12.0_f64 * t2631 * t8882 - 24.0_f64 * t2633 * t8865 + 3.0_f64 * t3431 * t597 - 12.0_f64 * t3435 * t594 + 3.0_f64 * t3438 * t594;
    (t8882, t8885, t8888)
}
