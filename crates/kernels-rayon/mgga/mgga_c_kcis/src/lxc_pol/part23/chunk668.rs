//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 668/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk668(t209: f64, t24: f64, t1299: f64, t637: f64, t1640: f64, t448: f64, t1300: f64, t2272: f64, t1598: f64, t3964: f64) -> (f64, f64, f64, f64, f64) {
    let t7783 = t209 * t24;
    let t7886 = t1299 * t637;
    let t7889 = t448 * t1640;
    let t7892 = t1300 * t2272;
    let t7895 = t3964 * t1598;
    (t7783, t7886, t7889, t7892, t7895)
}
