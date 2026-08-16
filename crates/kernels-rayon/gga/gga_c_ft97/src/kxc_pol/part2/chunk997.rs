//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 997/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk997(t10447: f64, t4261: f64, t309: f64, t799: f64, t1248: f64, t2842: f64, t2867: f64, t4152: f64, t8392: f64, t1882: f64, t4173: f64, t1212: f64, t2844: f64) -> (f64, f64, f64, f64, f64) {
    let t15455 = t10447 * t4261;
    let t15460 = t799 * t309;
    let t15461 = t2842 * t1248;
    let t15462 = t15461 * t2867;
    let t15463 = t15460 * t15462;
    let t15467 = 2.0_f64 / 27.0_f64 * t8392 * t4152;
    let t15471 = 2.0_f64 / 27.0_f64 * t1882 * t4173;
    let t15472 = t1212 * t2844;
    (t15455, t15463, t15467, t15471, t15472)
}
