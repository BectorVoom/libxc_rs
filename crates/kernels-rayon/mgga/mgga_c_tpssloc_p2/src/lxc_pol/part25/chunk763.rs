//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 763/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk763(t761: f64, t9905: f64, t9820: f64, t9824: f64, t9881: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64, t9896: f64, t9900: f64, t9903: f64) -> (f64, f64) {
    let t9907 = 0.35089341735807877242e1_f64 * t761 * t9905;
    let t9908 = -t9820 - t9824 + t9881 - t9884 + t9887 + t9890 - t9894 + t9896 + t9900 - t9903 + t9907;
    (t9907, t9908)
}
