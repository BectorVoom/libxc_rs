//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 600/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk600(t36: f64, t876: f64, t262: f64, t7835: f64, t265: f64, t352: f64, t2079: f64, t2067: f64, t3851: f64) -> (f64, f64, f64, f64) {
    let t7836 = t36 * t876;
    let t7838 = t7835 * t262 * t7836;
    let t7840 = t265 * t352;
    let t7842 = t2079 * t262 * t7840;
    let t7844 = t3851 * t2067;
    (t7838, t7840, t7842, t7844)
}
