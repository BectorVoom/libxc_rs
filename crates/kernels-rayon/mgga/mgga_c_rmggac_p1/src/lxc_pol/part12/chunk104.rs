//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 104/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk104(t84: f64, t280: f64, rho0: f64, tau0: f64) -> (f64, f64, f64) {
    let t306 = rho0 * rho0;
    let t308 = 1.0_f64 / t84 / t306;
    let t309 = tau0 * t308;
    let t312 = t280 / 2.0_f64;
    (t306, t309, t312)
}
