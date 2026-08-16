//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 529/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk529(t2160: f64, t2165: f64, t638: f64, t2169: f64, t1173: f64, t205: f64, t671: f64) -> (f64, f64, f64, f64) {
    let t7210 = t638 * t2160 * t2165;
    let t7213 = t638 * t2160 * t2169;
    let t7228 = t1173 * t205;
    let t7229 = t671 * t7228;
    (t7210, t7213, t7228, t7229)
}
