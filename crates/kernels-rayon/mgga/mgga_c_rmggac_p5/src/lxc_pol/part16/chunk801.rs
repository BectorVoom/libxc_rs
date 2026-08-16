//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 801/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk801(t1986: f64, t2318: f64, t305: f64, t321: f64, t49: f64, t529: f64, t36940: f64, t36945: f64, t68: f64, t2411: f64, t678: f64, t7920: f64) -> (f64, f64, f64, f64) {
    let t39103 = t1986 * t305 * t2318 * t321;
    let t39116 = t49 * t529;
    let t39119 = t36945 * t39116 * t68 * t36940;
    let t39122 = t2411 * t7920 * t678;
    (t39103, t39116, t39119, t39122)
}
