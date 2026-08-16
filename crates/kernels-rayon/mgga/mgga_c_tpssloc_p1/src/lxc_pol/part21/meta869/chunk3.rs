//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3185/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3185(t11692: f64, t11697: f64, t18964: f64, t18583: f64, t3577: f64, t11678: f64, t18367: f64, t1227: f64, t13969: f64, t18593: f64, t15640: f64, t15737: f64) -> (f64, f64, f64, f64, f64) {
    let t66073 = t11692 * t11697 * t18964;
    let t66076 = t3577 * t11697 * t18583;
    let t66079 = t11678 * t11697 * t18367;
    let t66084 = t1227 * t13969 * t18593;
    let t66092 = t15737 * t15640;
    (t66073, t66076, t66079, t66084, t66092)
}
