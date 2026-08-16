//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1807/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1807(t23109: f64, t2632: f64, t81914: f64, t81915: f64, t10024: f64, t1899: f64, t23110: f64, t232: f64, t23116: f64, t838: f64, t2693: f64, t6609: f64) -> (f64, f64, f64, f64, f64) {
    let t81918 = t23109 * t81914 * t81915 * t2632;
    let t81920 = t1899 * t10024;
    let t81924 = t23109 * t23110 * t81915 * t232;
    let t81926 = t23116 * t838;
    let t81928 = t6609 * t2693;
    (t81918, t81920, t81924, t81926, t81928)
}
