//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1247/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1247(t2138: f64, t2147: f64, t322: f64, t9417: f64, t1265: f64, t2146: f64, t33175: f64, t33320: f64, t33321: f64, t33324: f64, t38471: f64, t38474: f64, t38481: f64, t38487: f64, t38489: f64, t556: f64, t7912: f64, t7931: f64, t8301: f64, t9025: f64, t9136: f64, t9391: f64, t9414: f64, t9418: f64) -> f64 {
    let t38493 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t9417 * t322;
    let t38503 = -t38471 - 0.8673628188205199462e0_f64 * t38474 + 0.17347256376410398924e1_f64 * t7912 * t9418 + 0.17347256376410398924e1_f64 * t7912 * t9414 + t38481 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t8301 * t556 - t38487 + t38489 - t38493 - t33320 + 0.17347256376410398924e1_f64 * t33321 + 0.8673628188205199462e0_f64 * t7912 * t9136 - 0.13170898365871023197e1_f64 * t33324 - 0.17347256376410398924e1_f64 * t7931 * t33175 * t9025 - 0.65854491829355115987e0_f64 * t9391 * t1265;
    t38503
}
