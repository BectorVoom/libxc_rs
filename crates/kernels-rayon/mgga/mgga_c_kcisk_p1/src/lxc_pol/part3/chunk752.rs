//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 752/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk752(t1849: f64, t213: f64, t1060: f64, t3293: f64, t5136: f64, t1850: f64, t3290: f64, t4597: f64, t967: f64, t10487: f64, t167: f64, t11458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11612 = t213 * t1849;
    let t11613 = t11612 * t1060;
    let t11615 = t5136 * t3293;
    let t11623 = t1850 * t3290;
    let t11625 = t967 * t4597;
    let t11626 = t11625 * t3290;
    let t11630 = t167 * t10487;
    let t11633 = 0.71734315950379065738e-1_f64 * t11458;
    (t11613, t11615, t11623, t11626, t11630, t11633)
}
