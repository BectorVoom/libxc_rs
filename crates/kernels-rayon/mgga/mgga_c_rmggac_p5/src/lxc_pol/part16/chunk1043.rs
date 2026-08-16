//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1043/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1043(t2060: f64, t30283: f64, t903: f64, t30360: f64, t46502: f64, t7204: f64, t46358: f64, t8447: f64, t8577: f64, t8368: f64, t8533: f64, t1743: f64, t1971: f64, t495: f64, t511: f64, t7230: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47740 = t903 * t2060 * t30283;
    let t47743 = t903 * t2060 * t30360;
    let t47745 = t7204 * t46502;
    let t47747 = t7204 * t46358;
    let t47757 = t8577 * t8447;
    let t47759 = t8368 * t8533;
    let t47765 = t7230 * t1971 * t511 * t1743 * t495;
    (t47740, t47743, t47745, t47747, t47757, t47759, t47765)
}
