//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1249/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1249(t46978: f64, t7692: f64, t7690: f64, t93637: f64, t26807: f64, t7703: f64, t9938: f64, t26714: f64, t7696: f64, t26717: f64, t2173: f64, t10466: f64, t3489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93661 = t46978 * t7692;
    let t93662 = t7690 * t93661;
    let t93664 = t7690 * t93637;
    let t93686 = t7703 * t9938 * t26807;
    let t93690 = t7696 * t26714;
    let t93694 = t7696 * t26717;
    let t93704 = t2173 * t93637;
    let t93709 = t10466 * t3489;
    (t93661, t93662, t93664, t93686, t93690, t93694, t93704, t93709)
}
