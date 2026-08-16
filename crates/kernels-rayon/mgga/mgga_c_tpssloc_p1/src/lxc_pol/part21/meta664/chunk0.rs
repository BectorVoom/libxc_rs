//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2465/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2465(t1174: f64, t3442: f64, t44571: f64, t11588: f64, t3475: f64, t1176: f64, t697: f64, t1184: f64, t3447: f64, t3451: f64, t11153: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44573 = t1174 * t44571 * t3442;
    let t44579 = t11588 * t3475;
    let t44583 = t697 * t1176;
    let t44584 = t44583 * t1184;
    let t44586 = t3447 * t44584 * t3451;
    let t44607 = t460 * t11153;
    (t44573, t44579, t44583, t44584, t44586, t44607)
}
