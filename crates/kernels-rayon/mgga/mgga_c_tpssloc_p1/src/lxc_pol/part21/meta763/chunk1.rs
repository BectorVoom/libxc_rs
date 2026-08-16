//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2639/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2639(t16103: f64, t54670: f64, t16081: f64, t16090: f64, t16093: f64, t16097: f64, t2566: f64, t1307: f64, t16094: f64, t54665: f64, t686: f64, t16095: f64, t3719: f64) -> (f64, f64, f64, f64, f64) {
    let t54671 = t54670 * t16103;
    let t54673 = t16081 * t16090;
    let t54676 = t2566 * t16093 * t16097;
    let t54690 = t16094 * t686 * t54665 * t1307;
    let t54698 = t16094 * t686 * t16095 * t3719;
    (t54671, t54673, t54676, t54690, t54698)
}
