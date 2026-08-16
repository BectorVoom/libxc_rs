//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 773/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk773(t363: f64, t4431: f64, t3187: f64, t1909: f64, t3194: f64, t3193: f64, t11902: f64, t3205: f64, t11430: f64, t11436: f64, t11448: f64, t15978: f64, t15980: f64, t15983: f64, t15987: f64, t15991: f64, t15996: f64, t16000: f64, t16003: f64, t16008: f64, t1901: f64, t3281: f64, t446: f64) -> (f64, f64) {
    let t16011 = t4431 * t363;
    let t16012 = t3187 * t16011;
    let t16013 = t1909 * t16012;
    let t16016 = t3194 * t16011;
    let t16017 = t3193 * t16016;
    let t16020 = t11902 * t3205;
    let t16023 = t15978 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t15980 + t11430 - t11436 - t11448 - 2.0_f64 / 9.0_f64 * t446 * t15983 - 4.0_f64 / 9.0_f64 * t3281 * t15987 - t446 * t15991 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t15996 - 2.0_f64 / 3.0_f64 * t446 * t16000 - 4.0_f64 / 9.0_f64 * t1901 * t16003 + t1901 * t16008 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t16013 - 2.0_f64 / 27.0_f64 * t1901 * t16017 + 2.0_f64 / 9.0_f64 * t1901 * t16020;
    (t16011, t16023)
}
