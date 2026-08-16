//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 604/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk604(t1982: f64, t8512: f64, t205: f64, t4443: f64, t671: f64, t3350: f64) -> (f64, f64, f64, f64) {
    let t8513 = t8512 * t1982;
    let t8515 = t4443 * t205;
    let t8516 = t671 * t8515;
    let t8517 = t8516 * t3350;
    (t8513, t8515, t8516, t8517)
}
