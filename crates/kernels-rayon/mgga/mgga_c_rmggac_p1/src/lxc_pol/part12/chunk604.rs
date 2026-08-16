//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 604/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk604(t7810: f64, t793: f64, t7444: f64, t797: f64, t7707: f64, t128: f64, t830: f64) -> (f64, f64, f64, f64, f64) {
    let t7811 = t793 * t7810;
    let t7813 = t797 * t7444;
    let t7815 = t793 * t7707;
    let t7816 = 0.15965655602485078085e0_f64 * t7815;
    let t7817 = t128 * t830;
    (t7811, t7813, t7815, t7816, t7817)
}
