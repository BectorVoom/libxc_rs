//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1203/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1203(t1659: f64, t7973: f64, t1539: f64, t309: f64, t32181: f64, t36433: f64, t463: f64, t32003: f64, t2146: f64, t2147: f64, t32161: f64, t32163: f64, t32167: f64, t32168: f64, t32171: f64, t32176: f64, t32180: f64, t32183: f64, t32187: f64, t32191: f64, t5331: f64, t556: f64, t609: f64, t7877: f64) -> f64 {
    let t36473 = 0.13170898365871023197e1_f64 * t7973 * t1659;
    let t36475 = t1539 * t309;
    let t36477 = t32181 * t36433 * t36475;
    let t36479 = t1539 * t463;
    let t36482 = 0.34694512752820797848e1_f64 * t32003 * t36433 * t36479;
    let t36489 = -0.17347256376410398924e1_f64 * t32161 + 0.17347256376410398924e1_f64 * t32163 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t7877 * t556 - t32167 - 0.17347256376410398924e1_f64 * t32168 + 0.17347256376410398924e1_f64 * t32171 - t32176 + t32180 - t36473 - 0.69389025505641595696e1_f64 * t32183 - 0.34694512752820797848e1_f64 * t36477 + t36482 + 0.34694512752820797848e1_f64 * t32187 + 0.8673628188205199462e0_f64 * t32191 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t609 * t5331;
    t36489
}
