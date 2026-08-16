//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1959/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1959(t3701: f64, t6995: f64, t1307: f64, t2018: f64, t1862: f64, t31: f64, t1458: f64, t1868: f64, t7752: f64, t576: f64, t1409: f64, t1390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31035 = t3701 * t6995;
    let t31299 = t2018 * t1307;
    let t31682 = t1862 * t31;
    let t33085 = t1868 * t1458;
    let t33136 = t3701 * t7752;
    let t33185 = t576 * t1458;
    let t33567 = t31682 * t1409;
    let t34999 = t7752 * t1390;
    (t31035, t31299, t33085, t33136, t33185, t33567, t34999)
}
