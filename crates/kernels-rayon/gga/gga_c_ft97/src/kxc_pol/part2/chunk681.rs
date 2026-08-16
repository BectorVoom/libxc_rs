//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 681/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk681(t10491: f64, t309: f64, t2399: f64, t865: f64, t89: f64, t1882: f64, t2864: f64, t2850: f64, t2889: f64, t10394: f64, t10276: f64, t2787: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10492 = t10491 * t309;
    let t10514 = t89 * t2399 * t865;
    let t10533 = t1882 * t2864;
    let t10539 = t1882 * t2850;
    let t10545 = t1882 * t2889;
    let t10552 = t10394 / 3.0_f64;
    let t10555 = 2.0_f64 / 3.0_f64 * t10276;
    let t10559 = t458 * t2787;
    (t10492, t10514, t10533, t10539, t10545, t10552, t10555, t10559)
}
