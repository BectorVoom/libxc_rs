//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 992/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk992(t5299: f64, t615: f64, t310: f64, t8995: f64, t1221: f64, t2146: f64, t2341: f64, t30023: f64, t31965: f64, t31976: f64, t31978: f64, t31984: f64, t33635: f64, t33644: f64, t33648: f64, t33651: f64, t33656: f64, t621: f64, t7912: f64, t7931: f64, t7932: f64, t8014: f64, t8428: f64, t8441: f64, t9003: f64, t9033: f64) -> f64 {
    let t33658 = t615 * t5299;
    let t33662 = 0.13170898365871023197e1_f64 * t310 * t8995;
    let t33666 = 0.8673628188205199462e0_f64 * t7912 * t8428 + 0.69389025505641595696e1_f64 * t31976 + 0.34694512752820797848e1_f64 * t33635 + 0.10408353825846239354e2_f64 * t2146 * t30023 * t2341 * t1221 - 0.17347256376410398924e1_f64 * t31965 * t8441 + 0.17347256376410398924e1_f64 * t7931 * t9033 * t33644 - 0.65854491829355115987e0_f64 * t33648 - 0.8673628188205199462e0_f64 * t7931 * t7932 * t33651 - 0.17347256376410398924e1_f64 * t31978 + 0.65854491829355115987e0_f64 * t33656 - 0.4336814094102599731e0_f64 * t33658 * t621 + t33662 + 0.8673628188205199462e0_f64 * t9003 * t8014 + 0.65854491829355115987e0_f64 * t31984;
    t33666
}
