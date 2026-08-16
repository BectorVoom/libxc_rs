//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 271/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk271(t2127: f64, t82: f64, t290: f64, t668: f64, t507: f64, t511: f64, t321: f64, t649: f64, t27: f64) -> (f64, f64, f64, f64) {
    let t2128 = t82 * t2127;
    let t2131 = t290 * t668;
    let t2134 = t507 * t511;
    let t2135 = t649 * t321;
    let t2136 = t27 * t2135;
    (t2128, t2131, t2134, t2136)
}
