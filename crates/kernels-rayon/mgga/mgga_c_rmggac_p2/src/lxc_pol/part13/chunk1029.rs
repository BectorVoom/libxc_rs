//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1029/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1029(t9410: f64, t8677: f64, t8679: f64, t8681: f64, t8683: f64, t8685: f64, t8690: f64, t9423: f64, t9425: f64, t10357: f64, t10358: f64, t10359: f64, t10360: f64, t8172: f64) -> (f64, f64, f64, f64) {
    let t42500 = 0.11974241701863808564e0_f64 * t9410;
    let t42501 = 0.212822999466489197e-4_f64 * t8677;
    let t42502 = 0.1702583995731913576e-4_f64 * t8679;
    let t42504 = 0.5107751987195740728e-4_f64 * t8681;
    let t42505 = 0.5107751987195740728e-4_f64 * t8683;
    let t42506 = 0.1702583995731913576e-4_f64 * t8685;
    let t42507 = 0.1702583995731913576e-4_f64 * t8690;
    let t42508 = 0.79828278012425390428e-1_f64 * t9423;
    let t42509 = 0.39914139006212695214e-1_f64 * t9425;
    let t42510 = t42504 - t42505 - t42506 + t42507 + t10357 + t10358 - t10359 + t10360 + t42508 - t42509 - t8172;
    (t42500, t42501, t42502, t42510)
}
