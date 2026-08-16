//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 458/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk458(t1653: f64, t333: f64, t1598: f64, t866: f64, t571: f64, t833: f64, t325: f64, t623: f64, t4698: f64, t4700: f64, t4697: f64, t4705: f64) -> (f64, f64, f64, f64, f64) {
    let t4974 = t1653 * t333;
    let t4977 = t1598 * t866;
    let t4982 = t571 * t833;
    let t4985 = t623 * t325;
    let t4997 = 1584.0_f64 * t4698;
    let t4998 = 1872.0_f64 * t4700;
    let t4999 = t4697 - t4997 - t4998 + t4705;
    (t4974, t4977, t4982, t4985, t4999)
}
