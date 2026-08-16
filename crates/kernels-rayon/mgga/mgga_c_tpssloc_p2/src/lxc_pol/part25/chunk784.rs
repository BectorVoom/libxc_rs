//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 784/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk784(t10140: f64, t10143: f64, t193: f64, t202: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9872: f64, t9876: f64, t9881: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64, t9896: f64) -> f64 {
    let t10147 = 2.0_f64 * t10140 * t10143 * t193 * t202 + t9793 + t9797 - t9820 - t9824 + t9872 - t9876 + t9881 - t9884 + t9887 + t9890 - t9894 + t9896;
    t10147
}
