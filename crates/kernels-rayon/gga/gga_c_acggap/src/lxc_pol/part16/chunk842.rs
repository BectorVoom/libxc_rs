//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 842/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk842(t2354: f64, t814: f64, t813: f64, t1159: f64, t848: f64, t182: f64, t862: f64, t1016: f64, t360: f64, t1083: f64, t171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10956 = t814 * t2354;
    let t11882 = t813 * t813;
    let t11883 = 1.0_f64 / t11882;
    let t12726 = t848 * t1159;
    let t12935 = t862 * t182;
    let t13067 = t360 * t1016;
    let t13287 = t171 * t1083;
    (t10956, t11883, t12726, t12935, t13067, t13287)
}
