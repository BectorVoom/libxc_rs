//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 691/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk691(t1775: f64, t3135: f64, t3128: f64, t2: f64, t8275: f64, t11175: f64, t17: f64, t9: f64, t3141: f64, t8282: f64, t959: f64, t3151: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11684 = 4.0_f64 / 9.0_f64 * t1775 * t3135;
    let t11686 = 4.0_f64 / 27.0_f64 * t1775 * t3128;
    let t11690 = t8275 * t2;
    let t11717 = t9 * t11175 * t17;
    let t11718 = t11717 * t3141;
    let t11720 = t8282 * t959;
    let t11732 = 4.0_f64 / 3.0_f64 * t1775 * t3151;
    (t11684, t11686, t11690, t11717, t11718, t11720, t11732)
}
