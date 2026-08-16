//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1272/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1272(t33796: f64, t9168: f64, t2131: f64, t2132: f64, t309: f64, t9971: f64, t10017: f64, t2138: f64, t2147: f64, t322: f64, t157: f64, t1915: f64, t2146: f64, t2152: f64, t2338: f64, t33301: f64, t38455: f64, t38458: f64, t38471: f64, t38474: f64, t38481: f64, t38662: f64, t406: f64, t463: f64, t8004: f64, t8316: f64, t9422: f64, t9428: f64) -> f64 {
    let t42229 = t33796 * t9168;
    let t42247 = t2131 * t2132 * t9971 * t309;
    let t42252 = t2138 * t2147 * t10017 * t322;
    let t42256 = t2138 * t2132 * t9971 * t322;
    let t42258 = -0.17347256376410398924e1_f64 * t38662 * t9428 + 0.17347256376410398924e1_f64 * t42229 - t33301 + 0.69389025505641595696e1_f64 * t38455 + t38458 - 0.8673628188205199462e0_f64 * t2338 * t9422 - 0.26020884564615598386e1_f64 * t2146 * t8004 * t10017 * t463 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t9971 * t406 * t157 + 0.13170898365871023197e1_f64 * t8316 * t1915 - 0.8673628188205199462e0_f64 * t42247 - t38471 - 0.17347256376410398924e1_f64 * t38474 - 0.17347256376410398924e1_f64 * t42252 + t38481 + 0.8673628188205199462e0_f64 * t42256;
    t42258
}
