//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1121/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1121(t1071: f64, t1670: f64, t2630: f64, t3269: f64, t13480: f64, t4565: f64, t13475: f64, t4579: f64, t13511: f64, t3255: f64, t4597: f64, t1035: f64, t3293: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14155 = t3269 * t1670 * t1071 * t2630;
    let t14158 = t4565 * t13480;
    let t14161 = t4579 * t13475;
    let t14164 = t4579 * t13511;
    let t14168 = 0.13140859333333333333e-2_f64 * t3255 * t4597;
    let t14170 = t3293 * t1035;
    (t14155, t14158, t14161, t14164, t14168, t14170)
}
