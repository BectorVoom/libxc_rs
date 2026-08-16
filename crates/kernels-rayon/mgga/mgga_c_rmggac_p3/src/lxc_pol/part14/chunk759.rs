//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 759/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk759(t35607: f64, t1343: f64, t2019: f64, t2084: f64, t7765: f64, t1330: f64, t28: f64, t271: f64, t7553: f64, t7557: f64, t7491: f64, t7766: f64) -> (f64, f64, f64, f64) {
    let t35608 = 0.45731474687362542471e-3_f64 * t35607;
    let t35611 = t2019 * t2084 * t1343 * t7765;
    let t35612 = 0.24390119833260022651e-2_f64 * t35611;
    let t35613 = t28 * t1330;
    let t35616 = t7553 * t35613 * t271 * t7557;
    let t35617 = 0.5854811038705731867e-3_f64 * t35616;
    let t35618 = t7491 * t7766;
    (t35608, t35612, t35617, t35618)
}
