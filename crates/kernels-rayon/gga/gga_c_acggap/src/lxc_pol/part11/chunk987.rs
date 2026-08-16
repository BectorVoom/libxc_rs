//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 987/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk987(t1411: f64, t463: f64, t309: f64, t7932: f64, t7963: f64, t23688: f64, t7942: f64, t31895: f64, t31897: f64, t31901: f64, t31905: f64, t33518: f64, t33523: f64, t33525: f64, t33529: f64, t33533: f64, t33538: f64, t33541: f64, t33546: f64, t7931: f64) -> f64 {
    let t33547 = t1411 * t463;
    let t33551 = t1411 * t309;
    let t33554 = 0.17347256376410398924e1_f64 * t7963 * t7932 * t33551;
    let t33557 = 0.17347256376410398924e1_f64 * t7942 * t7932 * t23688;
    let t33558 = -0.8673628188205199462e0_f64 * t33518 + t33523 - 0.8673628188205199462e0_f64 * t33525 - t33529 - 0.8673628188205199462e0_f64 * t31895 - 0.17347256376410398924e1_f64 * t31897 + t33533 - 0.17347256376410398924e1_f64 * t31901 + t33538 - t33541 - 0.34694512752820797848e1_f64 * t31905 + t33546 - 0.17347256376410398924e1_f64 * t7931 * t7932 * t33547 + t33554 - t33557;
    t33558
}
