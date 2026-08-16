//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 773/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk773(t1073: f64, t8680: f64, t2267: f64, t1570: f64, t2266: f64, t643: f64, t920: f64, t363: f64, t1557: f64, t8654: f64, t2294: f64, t3640: f64, t637: f64) -> (f64, f64, f64, f64) {
    let t12112 = t8680 * t1073;
    let t12113 = t12112 * t2267;
    let t12116 = t2266 * t1570;
    let t12117 = t920 * t643;
    let t12118 = t12117 * t363;
    let t12119 = t12116 * t12118;
    let t12122 = t8654 * t1557;
    let t12123 = t12122 * t12118;
    let t12128 = t637 * t3640 * t2294;
    (t12113, t12119, t12123, t12128)
}
