//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 834/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk834(t3621: f64, t426: f64, t187: f64, t2997: f64, t1210: f64, t3573: f64, t396: f64, t3576: f64, t404: f64, t962: f64, t3031: f64, t956: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10819 = 1.0_f64 / t3621 / t426;
    let t10845 = t187 * t2997;
    let t10861 = 1.0_f64 / t3573 / t1210;
    let t10862 = t396 * t10861;
    let t10865 = 1.0_f64 / t3576 / t404;
    let t10869 = t2997 * t962;
    let t10874 = t956 * t3031;
    (t10819, t10845, t10862, t10865, t10869, t10874)
}
