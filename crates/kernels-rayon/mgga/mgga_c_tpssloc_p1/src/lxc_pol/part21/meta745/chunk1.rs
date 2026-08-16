//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2615/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2615(t1213: f64, t15525: f64, t248: f64, t3570: f64, t11813: f64, t5018: f64, t15749: f64, t3577: f64, t45124: f64, t11835: f64, t4889: f64, t1174: f64, t1725: f64, t2402: f64) -> (f64, f64, f64, f64, f64) {
    let t53404 = t1213 * t248 * t3570 * t15525;
    let t53406 = t11813 * t5018;
    let t53410 = t3577 * t45124 * t15749;
    let t53433 = t4889 * t11835;
    let t53440 = t1174 * t2402 * t1725;
    (t53404, t53406, t53410, t53433, t53440)
}
