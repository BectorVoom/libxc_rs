//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 726/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk726(t1985: f64, t7799: f64, t606: f64, t7610: f64, t1994: f64, t137: f64, t5: f64) -> (f64, f64, f64, f64) {
    let t7800 = t7799 * t1985;
    let t7801 = 0.14291339372689912324e-3_f64 * t7800;
    let t7802 = t7610 * t606;
    let t7803 = 0.15724046144802076034e-3_f64 * t7802;
    let t7805 = t7799 * t1994;
    let t7806 = 0.20965394859736101378e-3_f64 * t7805;
    let t7815 = t5 * t137;
    (t7801, t7803, t7806, t7815)
}
