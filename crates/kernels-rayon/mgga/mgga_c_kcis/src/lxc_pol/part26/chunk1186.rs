//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1186/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1186(t4413: f64, t7490: f64, t1591: f64, t23024: f64, t4479: f64, t7533: f64, t1385: f64, t60029: f64, t1610: f64, t6284: f64, t167: f64, t2104: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62417 = t7490 * t4413;
    let t62923 = t23024 * t1591;
    let t63256 = t7533 * t4479;
    let t75638 = t60029 * t1385;
    let t77072 = t6284 * t1610;
    let t77753 = t2104 * t167;
    (t62417, t62923, t63256, t75638, t77072, t77753)
}
