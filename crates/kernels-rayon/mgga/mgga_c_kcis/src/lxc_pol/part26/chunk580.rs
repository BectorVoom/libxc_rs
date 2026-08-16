//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 580/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk580(t6010: f64, t6012: f64, t1466: f64, t2033: f64, t1535: f64, t552: f64, t5869: f64, t577: f64, t585: f64, t1539: f64, t2035: f64, t1543: f64, t2062: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6013 = t6010 * t6012;
    let t6015 = t2033 * t1466;
    let t6016 = t6015 * sigma2;
    let t6017 = t6016 * t1535;
    let t6019 = t5869 * t552;
    let t6020 = t6019 * t577;
    let t6021 = t6020 * t585;
    let t6023 = t2035 * t1539;
    let t6025 = t1543 * t2062;
    (t6013, t6015, t6016, t6017, t6019, t6020, t6021, t6023, t6025)
}
