//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 466/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk466(t2097: f64, t4511: f64, t2102: f64, t4656: f64, t4518: f64, t582: f64, t4522: f64, t2112: f64, t24: f64, t4668: f64, t4714: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4759 = t2097 * t4511;
    let t4762 = t2102 * t4656;
    let t4765 = t582 * t4518;
    let t4768 = t582 * t4522;
    let t4772 = t24 * t2112 * t4668;
    let t4776 = t24 * t586 * t4714;
    (t4759, t4762, t4765, t4768, t4772, t4776)
}
