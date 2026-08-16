//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1133/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1133(t1038: f64, t28622: f64, t311: f64, t4043: f64, t6851: f64, t1026: f64, t1093: f64, t2153: f64, t11417: f64, t11971: f64, t761: f64, t1645: f64, t189: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t34013 = t311 * t6851 * t4043 * pi * t1038 * t28622;
    let t34016 = t2153 * t1026 * t1093;
    let t34019 = t761 * t11417 * t11971;
    let t34021 = t189 * t1645;
    (t34013, t34016, t34019, t34021)
}
