//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 811/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk811(t2147: f64, t8232: f64, t1591: f64, t2666: f64, t1568: f64, t8089: f64, t7623: f64, t2214: f64, t2698: f64, t514: f64, t1616: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8234 = 0.11643651550782197811e-1_f64 * t2147 * t8232;
    let t8240 = t1591 * t2666;
    let t8243 = t1568 * t8089;
    let t8245 = 0.10975748638225852664e-1_f64 * t7623 * t8243;
    let t8263 = t2214 * t2698;
    let t8265 = 0.19514881078765566037e-1_f64 * t514 * t8263;
    let t8266 = t1616 * t938;
    (t8234, t8240, t8243, t8245, t8265, t8266)
}
