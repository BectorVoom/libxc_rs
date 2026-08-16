//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 826/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk826(t120: f64, t929: f64, t3056: f64, t72: f64, t4441: f64, t71: f64, t530: f64, t383: f64, t4690: f64, t1005: f64, t4693: f64, t126: f64, t15776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16858 = t929 * t120;
    let t16860 = t72 * t16858 * t3056;
    let t16863 = t71 * t4441;
    let t16864 = t16863 * t530;
    let t16867 = t4690 * t383;
    let t16870 = t1005 * t3056;
    let t16875 = t4693 * t383;
    let t16878 = t15776 * t126;
    (t16860, t16864, t16867, t16870, t16875, t16878)
}
