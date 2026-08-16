//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 967/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk967(t1232: f64, t1771: f64, t4224: f64, t458: f64, t11717: f64, t4210: f64, t10261: f64, t2682: f64, t4218: f64, t2681: f64, t2739: f64, t1228: f64, t8282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15011 = t1771 * t1232;
    let t15014 = 2.0_f64 / 3.0_f64 * t458 * t4224;
    let t15015 = t11717 * t4210;
    let t15018 = t10261 * t4218 * t2682;
    let t15022 = t2681 * t4218 * t2739;
    let t15025 = t8282 * t1228;
    (t15011, t15014, t15015, t15018, t15022, t15025)
}
