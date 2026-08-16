//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 838/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk838(t11593: f64, t12676: f64, t16979: f64, t16983: f64, t16986: f64, t16990: f64, t16993: f64, t16998: f64, t17003: f64, t17008: f64, t17013: f64, t17018: f64, t17023: f64, t17027: f64, t17032: f64, t17035: f64, t1901: f64, t446: f64) -> f64 {
    let t17038 = 2.0_f64 / 3.0_f64 * t446 * t16979 - t446 * t16983 / 9.0_f64 + t12676 - 2.0_f64 / 27.0_f64 * t16986 + 4.0_f64 / 9.0_f64 * t11593 * t16990 + 2.0_f64 / 9.0_f64 * t1901 * t16993 + 2.0_f64 / 9.0_f64 * t1901 * t16998 + 4.0_f64 / 9.0_f64 * t11593 * t17003 + t1901 * t17008 / 9.0_f64 + t1901 * t17013 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t17018 - 2.0_f64 / 9.0_f64 * t1901 * t17023 - 4.0_f64 / 3.0_f64 * t1901 * t17027 - 4.0_f64 / 3.0_f64 * t1901 * t17032 + 2.0_f64 / 9.0_f64 * t1901 * t17035;
    t17038
}
