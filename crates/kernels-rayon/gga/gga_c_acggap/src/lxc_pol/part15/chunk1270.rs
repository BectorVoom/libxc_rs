//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1270/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1270(t32003: f64, t38052: f64, t8406: f64, t1915: f64, t8331: f64, t2222: f64, t2236: f64, t33256: f64, t33266: f64, t33271: f64, t38386: f64, t38389: f64, t38392: f64, t38393: f64, t38397: f64, t38415: f64, t38418: f64, t40620: f64, t6425: f64) -> f64 {
    let t42194 = t32003 * t38052 * t8406;
    let t42200 = t8331 * t1915;
    let t42205 = -t38386 + t38389 - t38392 + 0.13170898365871023197e1_f64 * t38393 + 0.34694512752820797848e1_f64 * t42194 - 0.8673628188205199462e0_f64 * t33256 + 0.26341796731742046394e1_f64 * t2222 * t6425 - 0.26341796731742046394e1_f64 * t38397 + 0.13170898365871023197e1_f64 * t42200 + 0.8673628188205199462e0_f64 * t40620 * t2236 - t38415 + t38418 - 0.8673628188205199462e0_f64 * t33266 - t33271;
    t42205
}
