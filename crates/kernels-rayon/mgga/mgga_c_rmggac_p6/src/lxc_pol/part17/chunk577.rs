//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 577/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk577(t305: f64, t7817: f64, t648: f64, t7561: f64, t2068: f64, t7638: f64, t2067: f64, t3839: f64) -> (f64, f64, f64, f64) {
    let t7818 = t305 * t7817;
    let t7819 = 0.14635184302277988245e0_f64 * t7818;
    let t7820 = t648 * t7561;
    let t7821 = 0.33335697577410973224e-1_f64 * t7820;
    let t7826 = t2068 * t7638;
    let t7829 = t3839 * t2067;
    (t7819, t7821, t7826, t7829)
}
