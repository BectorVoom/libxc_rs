//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 850/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk850(t34938: f64, t5149: f64, t656: f64, t1550: f64, t2060: f64, t27059: f64, t2347: f64, t876: f64, t262: f64, t7501: f64, t8672: f64, t321: f64, t8704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39039 = t34938 * t656 * t5149;
    let t39042 = t1550 * t2060 * t27059;
    let t39044 = t2347 * t876;
    let t39045 = t262 * t39044;
    let t39046 = t34938 * t39045;
    let t39048 = t7501 * t8672;
    let t39055 = t8704 * t321;
    (t39039, t39042, t39044, t39045, t39046, t39048, t39055)
}
