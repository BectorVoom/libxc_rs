//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 900/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk900(t27: f64, t8464: f64, t221: f64, t2485: f64, t257: f64, t786: f64, t7063: f64, t1032: f64, t1955: f64) -> (f64, f64, f64, f64) {
    let t31743 = t8464 * t27;
    let t31746 = t2485 * t221 * t257;
    let t31747 = t786 * t31743 * t31746;
    let t31748 = 0.18822977838986977999e-4_f64 * t31747;
    let t31750 = t7063 * t31743 * t31746;
    let t31751 = 0.33467254597718846885e-4_f64 * t31750;
    let t31752 = t1955 * t1032;
    (t31746, t31748, t31751, t31752)
}
