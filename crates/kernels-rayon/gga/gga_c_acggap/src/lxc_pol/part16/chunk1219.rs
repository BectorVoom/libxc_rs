//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1219/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1219(t315: f64, t323: f64, t9767: f64, t2138: f64, t2147: f64, t322: f64, t9789: f64, t8419: f64, t8998: f64, t1620: f64, t2146: f64, t2338: f64, t30023: f64, t32187: f64, t32196: f64, t32210: f64, t36477: f64, t36482: f64, t36498: f64, t36504: f64, t463: f64, t8415: f64, t9003: f64, t9010: f64, t9015: f64, t9044: f64, t9497: f64) -> f64 {
    let t40884 = t315 * t9767 * t323;
    let t40895 = t2138 * t2147 * t9789 * t322;
    let t40905 = t8998 * t8419;
    let t40907 = -0.69389025505641595696e1_f64 * t36477 + t36482 - 0.65854491829355115987e0_f64 * t40884 + 0.17347256376410398924e1_f64 * t32187 + 0.17347256376410398924e1_f64 * t9003 * t8415 + 0.26341796731742046394e1_f64 * t9010 * t1620 + 0.8673628188205199462e0_f64 * t9003 * t9015 - 0.34694512752820797848e1_f64 * t40895 - t36498 + 0.8673628188205199462e0_f64 * t32196 - 0.13170898365871023197e1_f64 * t36504 - 0.8673628188205199462e0_f64 * t2338 * t9044 - t32210 + 0.10408353825846239354e2_f64 * t2146 * t30023 * t9497 * t463 + 0.17347256376410398924e1_f64 * t40905;
    t40907
}
