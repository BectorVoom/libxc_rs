//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1481/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1481(t15292: f64, t15330: f64, t15386: f64, t15423: f64, t225: f64, t3507: f64, t475: f64, t6739: f64, t1755: f64, t11546: f64, t14726: f64, t15026: f64, t3032: f64) -> (f64, f64, f64, f64, f64) {
    let t15425 = t15292 + t15330 + t15386 + t15423;
    let t15426 = t15425 * t225;
    let t15429 = t6739 * t3507 * t475;
    let t15430 = t1755 * t15429;
    let t15434 = t11546 * t14726;
    let t15437 = t15026 * t3032;
    (t15425, t15426, t15430, t15434, t15437)
}
