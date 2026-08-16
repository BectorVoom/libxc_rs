//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 863/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk863(t38775: f64, t38818: f64, t38837: f64, t38853: f64, t38857: f64, t38860: f64, t38863: f64, t38869: f64, t38872: f64, t38881: f64, t38886: f64, t38934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42740 = 0.36366215538993788974e-1_f64 * t38775;
    let t42749 = 0.1440846329149835838e-2_f64 * t38818;
    let t42755 = 0.1440846329149835838e-2_f64 * t38837;
    let t42759 = 0.20496175532535769482e-3_f64 * t38853;
    let t42760 = 0.1440846329149835838e-2_f64 * t38857;
    let t42761 = 0.1440846329149835838e-2_f64 * t38860;
    let t42762 = 0.1440846329149835838e-2_f64 * t38863;
    let t42764 = 0.20496175532535769482e-3_f64 * t38869;
    let t42767 = 0.20496175532535769482e-3_f64 * t38872;
    let t42771 = 0.86737941314158990616e-4_f64 * t38881;
    let t42772 = 0.86737941314158990616e-4_f64 * t38886;
    let t42785 = 0.11918087970123395032e-3_f64 * t38934;
    (t42740, t42749, t42755, t42759, t42760, t42761, t42762, t42764, t42767, t42771, t42772, t42785)
}
