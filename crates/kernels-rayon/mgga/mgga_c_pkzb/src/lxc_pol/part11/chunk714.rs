//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 714/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk714(t1721: f64, t600: f64, t173: f64, t607: f64, t614: f64, t1730: f64) -> (f64, f64, f64, f64) {
    let t5250 = t600 * t1721;
    let t5255 = t607 * t173;
    let t5256 = t5255 * t614;
    let t5257 = t1730 * t5256;
    (t5250, t5255, t5256, t5257)
}
