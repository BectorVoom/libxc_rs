//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1801/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1801(t23098: f64, t81782: f64, t81783: f64, t22822: f64, t281: f64, t6589: f64, t23124: f64, t23076: f64, t6597: f64, t22690: f64, t2379: f64, t841: f64) -> (f64, f64, f64, f64, f64) {
    let t81785 = t81782 * t81783 * t23098;
    let t81788 = t22822 * t6589 * t281;
    let t81789 = t81788 * t23124;
    let t81792 = t6597 * t23076 * t281;
    let t81795 = t81792 * t22690 * t841 * t2379;
    (t81785, t81788, t81789, t81792, t81795)
}
