//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 879/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk879(t241: f64, t6589: f64, t67: f64, t2613: f64, t68: f64, t816: f64, t2632: f64, t2678: f64, t815: f64, t836: f64, t812: f64, t2649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9607 = t241 * t6589 * t67;
    let t9612 = t2613 * t68;
    let t9613 = t9612 * t816;
    let t9632 = t2632 * t2678;
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    let t9639 = t9638 * t2649;
    (t9607, t9612, t9613, t9632, t9638, t9639)
}
