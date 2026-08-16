//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 948/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk948(t1619: f64, t309: f64, t33743: f64, t620: f64, t2131: f64, t2147: f64, t2341: f64, t847: f64, t2331: f64, t862: f64, t865: f64, t1219: f64, t615: f64, t8396: f64) -> (f64, f64, f64, f64) {
    let t33744 = t1619 * t309;
    let t33747 = 0.10408353825846239354e2_f64 * t33743 * t620 * t33744;
    let t33767 = t2131 * t2147 * t2341 * t847;
    let t33771 = t862 * t2331 * t865;
    let t33778 = t615 * t8396 * t1219;
    (t33747, t33767, t33771, t33778)
}
