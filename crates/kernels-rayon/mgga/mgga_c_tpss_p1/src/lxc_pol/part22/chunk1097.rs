//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1097/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1097(t11844: f64, t11873: f64, t11857: f64, t11860: f64, t11862: f64, t11865: f64, t11867: f64, t11871: f64, t11875: f64, t11880: f64, t11885: f64, t11890: f64) -> (f64, f64) {
    let t12024 = 0.13892666666666666667e0_f64 * t11844;
    let t12035 = 0.22954444444444444444e0_f64 * t11873;
    let t12040 = -0.157790625e0_f64 * t11857 - 0.3529725e1_f64 * t11860 - 0.17648625e1_f64 * t11862 + 0.6311625e0_f64 * t11865 + 0.31558125e0_f64 * t11867 + 0.62517e0_f64 * t11871 + t12035 - 0.68863333333333333333e0_f64 * t11875 + 0.57386111111111111112e0_f64 * t11880 - 0.20659e1_f64 * t11885 - 0.68863333333333333334e0_f64 * t11890;
    (t12024, t12040)
}
