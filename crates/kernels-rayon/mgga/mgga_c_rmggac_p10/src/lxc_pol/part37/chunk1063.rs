//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1063/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1063(t74779: f64, t68911: f64, t71151: f64, t74772: f64, t74775: f64, t74782: f64, t74786: f64, t77193: f64, t77195: f64, t77196: f64, t77197: f64, t77206: f64, t77208: f64, t77212: f64, t77213: f64, t77214: f64, t77218: f64) -> f64 {
    let t80158 = 0.65053455985619242964e-5_f64 * t74779;
    let t80160 = -t77193 - 0.1313947956967602539e-5_f64 * t74772 - t77195 - t77196 + t77197 - 0.43798265232253417968e-6_f64 * t74775 - t68911 + t80158 - t74782 + t71151 - t77206 + 0.72714524817717142305e-5_f64 * t74786 - t77208 - t77212 - t77213 + t77214 - t77218;
    t80160
}
