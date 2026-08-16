//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 647/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk647(t4581: f64, t5142: f64, t1154: f64, t1155: f64, t167: f64, t1791: f64, t238: f64, t86: f64, t1745: f64, t330: f64, t829: f64, t304: f64, t4920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5143 = t5142 * t4581;
    let t5147 = t1154 * t1155 * t167;
    let t5151 = t86 * t238 * t1791;
    let t5153 = t1745 * t330;
    let t5155 = t1154 * t5153 * t829;
    let t5158 = t304 * t4920;
    (t5143, t5147, t5151, t5153, t5155, t5158)
}
