//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1050/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1050(t73234: f64, t74197: f64, t74199: f64, t74207: f64, t74209: f64, t74213: f64, t74217: f64, t74225: f64, t76897: f64, t76898: f64, t76904: f64, t76913: f64, t76918: f64, t76923: f64, t76925: f64, t76927: f64, t76928: f64) -> f64 {
    let t80066 = -t76897 - t76898 - 0.57000320883372412499e-7_f64 * t74197 - 0.57000320883372412499e-7_f64 * t74199 + t76904 + 0.58171619854173713844e-5_f64 * t74207 - 0.58171619854173713844e-5_f64 * t74209 + 0.58171619854173713844e-5_f64 * t74213 - 0.17451485956252114153e-4_f64 * t74217 - t76913 - 0.2363e1_f64 * t73234 + t76918 + t76923 + t76925 - t76927 + t76928 - t74225;
    t80066
}
