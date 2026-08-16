//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 744/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk744(t1775: f64, t3135: f64, t3128: f64, t11034: f64, t3127: f64, t2: f64, t8275: f64, t11008: f64, t11013: f64, t11017: f64, t1787: f64, t11059: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11684 = 4.0_f64 / 9.0_f64 * t1775 * t3135;
    let t11686 = 4.0_f64 / 27.0_f64 * t1775 * t3128;
    let t11687 = t3127 * t11034;
    let t11690 = t8275 * t2;
    let t11691 = t11690 * t11008;
    let t11694 = t3127 * t11013;
    let t11697 = t1787 * t11017;
    let t11700 = t3127 * t11059;
    (t11684, t11686, t11687, t11691, t11694, t11697, t11700)
}
