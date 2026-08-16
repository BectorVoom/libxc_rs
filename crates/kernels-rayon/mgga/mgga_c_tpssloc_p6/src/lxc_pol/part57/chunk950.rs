//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 950/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk950(t1410: f64, t7440: f64, t8308: f64, t1433: f64, t2240: f64, t32: f64, t5392: f64, t27948: f64, t8307: f64, t8513: f64, t28007: f64, t8326: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126070 = t8308 * t1410 * t7440;
    let t126073 = t1410 * t1433;
    let t126091 = t2240 * t32 * t5392;
    let t126100 = t8513 * t8307 * t27948;
    let t126103 = t1433 * t1433;
    let t126116 = 2.0_f64 * t28007 * t8326;
    (t126070, t126073, t126091, t126100, t126103, t126116)
}
