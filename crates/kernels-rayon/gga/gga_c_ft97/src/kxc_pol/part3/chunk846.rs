//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 846/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk846(t1017: f64, t3590: f64, t574: f64, t4839: f64, t558: f64, t12680: f64, t3430: f64, t3435: f64, t1045: f64, t2097: f64, t3441: f64, t1060: f64, t3408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17151 = t574 * t3590 * t1017;
    let t17155 = t574 * t4839 * t558;
    let t17158 = t12680 * t3430;
    let t17161 = t12680 * t3435;
    let t17164 = t2097 * t1045;
    let t17165 = t17164 * t3441;
    let t17170 = t574 * t1060 * t3408;
    (t17151, t17155, t17158, t17161, t17165, t17170)
}
