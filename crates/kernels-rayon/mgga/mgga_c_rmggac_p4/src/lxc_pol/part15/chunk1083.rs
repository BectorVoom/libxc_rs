//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1083/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1083(t8447: f64, t8577: f64, t8368: f64, t8533: f64, t10115: f64, t275: f64, t1743: f64, t1971: f64, t495: f64, t511: f64, t7230: f64, t34847: f64, t9990: f64) -> (f64, f64, f64, f64, f64) {
    let t47757 = t8577 * t8447;
    let t47759 = t8368 * t8533;
    let t47761 = t275 * t10115;
    let t47765 = t7230 * t1971 * t511 * t1743 * t495;
    let t47767 = t34847 * t9990;
    (t47757, t47759, t47761, t47765, t47767)
}
