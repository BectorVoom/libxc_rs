//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 217/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk217(t664: f64, t685: f64, t650: f64, t657: f64) -> (f64, f64, f64) {
    let t687 = 1.0_f64 * t664 * t685;
    let t688 = 0.17123333333333333333e-1_f64 * t650;
    let t690 = -t688 + 0.5137e-1_f64 * t657;
    (t687, t688, t690)
}
