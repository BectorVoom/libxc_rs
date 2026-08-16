//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1385/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1385(t33930: f64, t33932: f64, t33937: f64, t33939: f64, t33941: f64, t33943: f64, t33946: f64, t33949: f64, t33952: f64, t33935: f64, t36761: f64, t33956: f64) -> (f64, f64) {
    let t36762 = 0.1011909669415296852e-6_f64 * t33930;
    let t36763 = 0.11372686522837130914e-4_f64 * t33932;
    let t36765 = 0.22509399720615334744e-7_f64 * t33937;
    let t36766 = 0.22509399720615334744e-6_f64 * t33939;
    let t36767 = 0.33147827249531850013e-7_f64 * t33941;
    let t36768 = 0.66295654499063700026e-7_f64 * t33943;
    let t36769 = 0.33147827249531850013e-7_f64 * t33946;
    let t36770 = 0.13913017666225690434e-3_f64 * t33949;
    let t36771 = 0.69504740211613770836e-3_f64 * t33952;
    let t36772 = t36761 - t36762 + t36763 + 0.25301106770833333336e-5_f64 * t33935 + t36765 + t36766 + t36767 + t36768 + t36769 - t36770 - t36771;
    let t36773 = 0.67402122125348062552e-7_f64 * t33956;
    (t36772, t36773)
}
