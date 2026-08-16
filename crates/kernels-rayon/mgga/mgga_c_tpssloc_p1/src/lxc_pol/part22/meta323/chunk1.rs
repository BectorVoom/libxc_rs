//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1509/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1509(t2535: f64, t5154: f64, t5166: f64, t592: f64, t12461: f64, t1845: f64, t118: f64, t1787: f64) -> (f64, f64, f64, f64) {
    let t15895 = t5154 * t2535;
    let t15898 = 8.0_f64 * t592 * t5166;
    let t15899 = t1845 * t12461;
    let t15908 = t1787 * t118;
    (t15895, t15898, t15899, t15908)
}
