//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 688/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk688(t2253: f64, t2930: f64, t703: f64, t900: f64, t230: f64, t2938: f64, t9556: f64, t2937: f64, t325: f64, t8640: f64, t895: f64, t2934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10843 = t2253 * t2930;
    let t10845 = t703 * t900;
    let t10864 = t230 * t2938;
    let t10883 = 0.44934037037037037036e0_f64 * t9556;
    let t10904 = 1.0_f64 / t2937 / t325;
    let t10921 = t8640 * t895;
    let t10923 = t2253 * t2934;
    (t10843, t10845, t10864, t10883, t10904, t10921, t10923)
}
