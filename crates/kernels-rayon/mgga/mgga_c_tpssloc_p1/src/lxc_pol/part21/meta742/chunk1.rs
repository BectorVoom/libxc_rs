//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2609/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2609(t1227: f64, t13969: f64, t15616: f64, t14706: f64, t248: f64, t3521: f64, t11814: f64, t4997: f64, t15492: f64, t3536: f64, t11692: f64, t11697: f64, t15703: f64) -> (f64, f64, f64, f64, f64) {
    let t53102 = t1227 * t13969 * t15616;
    let t53114 = t1227 * t248 * t3521 * t14706;
    let t53116 = t11814 * t4997;
    let t53118 = t3536 * t15492;
    let t53135 = t11692 * t11697 * t15703;
    (t53102, t53114, t53116, t53118, t53135)
}
