//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 420/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk420(t4103: f64, t5: f64, t12: f64, t3: f64, t154: f64, t4132: f64, t963: f64, t368: f64, t4135: f64, t142: f64, t265: f64, t6: f64) -> (f64, f64, f64, f64, f64) {
    let t4138 = t5 * t4103;
    let t4140 = 1.0_f64/pow_3_2(t12);
    let t4141 = t4140 * t3;
    let t4142 = t4141 * t154;
    let t4144 = t963 * t4132;
    let t4146 = t368 * t4135;
    let t4149 = t142 * t6 * t265;
    (t4138, t4142, t4144, t4146, t4149)
}
