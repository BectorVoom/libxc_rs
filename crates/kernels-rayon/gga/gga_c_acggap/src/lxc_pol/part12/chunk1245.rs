//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1245/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1245(t157: f64, t2146: f64, t2152: f64, t2338: f64, t2395: f64, t29994: f64, t33293: f64, t33294: f64, t33301: f64, t33306: f64, t33308: f64, t33675: f64, t38209: f64, t38383: f64, t38441: f64, t38443: f64, t38453: f64, t38455: f64, t38458: f64, t7912: f64, t7931: f64, t8303: f64, t9386: f64) -> f64 {
    let t38469 = -t33293 + t38441 + 0.8673628188205199462e0_f64 * t38443 + 0.65854491829355115987e0_f64 * t33294 + 0.34694512752820797848e1_f64 * t7931 * t38383 * t33675 + 0.8673628188205199462e0_f64 * t29994 * t2395 + 0.17347256376410398924e1_f64 * t38453 - t33301 + 0.34694512752820797848e1_f64 * t38455 + t38458 - 0.4336814094102599731e0_f64 * t2338 * t8303 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t38209 * t157 - 0.34694512752820797848e1_f64 * t33306 + 0.8673628188205199462e0_f64 * t7912 * t9386 + 0.13170898365871023197e1_f64 * t33308;
    t38469
}
