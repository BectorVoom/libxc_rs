//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 517/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk517(t158: f64, t2670: f64, t1054: f64, t1790: f64, t633: f64, t1717: f64, t183: f64) -> (f64, f64, f64, f64) {
    let t2671 = t2670 * t158;
    let t2678 = t1790 * t1054;
    let t2679 = t2678 * t633;
    let t2682 = t1717 * t183;
    (t2671, t2678, t2679, t2682)
}
