//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 731/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk731(t10580: f64, t2: f64, t1775: f64, t4215: f64, t1232: f64, t1771: f64, t4224: f64, t458: f64, t11717: f64, t4210: f64, t1228: f64, t8282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14961 = t10580 * t2;
    let t14999 = 2.0_f64 / 9.0_f64 * t1775 * t4215;
    let t15011 = t1771 * t1232;
    let t15014 = 2.0_f64 / 3.0_f64 * t458 * t4224;
    let t15015 = t11717 * t4210;
    let t15025 = t8282 * t1228;
    (t14961, t14999, t15011, t15014, t15015, t15025)
}
