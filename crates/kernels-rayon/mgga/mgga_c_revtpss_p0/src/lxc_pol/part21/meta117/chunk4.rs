//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 761/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk761(t213: f64, t2437: f64, t2443: f64, t2446: f64, t2449: f64, t2460: f64, t2462: f64, t2468: f64, t2473: f64, t257: f64, t2761: f64, t2765: f64, t2772: f64, t2829: f64, t865: f64, t887: f64) -> f64 {
    let t2832 = t2437 - t2443 - 0.10975748638225852664e-1_f64 * t2446 + 0.10975748638225852664e-1_f64 * t2449 + t2460 + 0.19514881078765566038e-1_f64 * t2462 - 0.19514881078765566038e-1_f64 * t2468 - t2473 + 0.65854491829355115987e0_f64 * t213 * t2761 * t257 - 0.13170898365871023197e1_f64 * t2765 * t887 + 0.13170898365871023197e1_f64 * t865 * t2772 - 0.65854491829355115987e0_f64 * t865 * t2829;
    t2832
}
