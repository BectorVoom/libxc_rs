//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 566/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk566(t1409: f64, t2770: f64, t607: f64, t2768: f64, t123: f64, t2775: f64, t882: f64, t3966: f64, t883: f64, t2765: f64, t2766: f64, t4335: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4337 = t2770 * t1409;
    let t4338 = t4337 * t607;
    let t4339 = t2768 * t4338;
    let t4340 = t123 * t4339;
    let t4342 = t2775 * t1409;
    let t4343 = t4342 * t607;
    let t4344 = t882 * t4343;
    let t4345 = t123 * t4344;
    let t4347 = t883 * t3966;
    let t4348 = t882 * t4347;
    let t4349 = t123 * t4348;
    let t4351 = t2765 + 0.5936111111111111111e-2_f64 * t2766 + 0.5936111111111111111e-2_f64 * t4335 - 0.11872222222222222222e-1_f64 * t4340 + 0.35616666666666666666e-1_f64 * t4345 - 0.17808333333333333333e-1_f64 * t4349;
    (t4338, t4340, t4343, t4345, t4347, t4349, t4351)
}
