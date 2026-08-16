//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 886/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk886(t1528: f64, t1970: f64, t209: f64, t236: f64, t605: f64, t7231: f64, t1494: f64, t618: f64, t10078: f64, t7255: f64, t1587: f64, t3352: f64) -> (f64, f64, f64, f64) {
    let t44676 = t1970 * t7231 * t236 * t1528 * t605 * t209;
    let t44682 = t1970 * t7231 * t236 * t618 * t1494 * t209;
    let t44684 = t7255 * t10078;
    let t44690 = t1970 * t3352 * t236 * t1587 * t605 * t209;
    (t44676, t44682, t44684, t44690)
}
