//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 815/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk815(t2152: f64, t9502: f64, t157: f64, t524: f64, t556: f64, t7932: f64, t2146: f64, t2338: f64, t2347: f64, t2351: f64, t7889: f64, t7900: f64, t7921: f64, t7926: f64, t7929: f64, t7931: f64, t8398: f64, t8408: f64, t8420: f64, t8424: f64, t8999: f64, t9003: f64, t9008: f64, t9018: f64, t9031: f64, t9493: f64, t9498: f64) -> (f64, f64, f64, f64) {
    let t9503 = t2152 * t9502;
    let t9508 = t556 * t524 * t157;
    let t9509 = t7932 * t9508;
    let t9514 = -0.17347256376410398924e1_f64 * t8398 + 0.8673628188205199462e0_f64 * t9003 * t2347 - 0.17347256376410398924e1_f64 * t8408 - t7889 + 0.17347256376410398924e1_f64 * t8420 - 0.17347256376410398924e1_f64 * t8424 + 0.17347256376410398924e1_f64 * t8999 - 0.8673628188205199462e0_f64 * t2146 * t9493 + t7900 + 0.13170898365871023197e1_f64 * t9008 - 0.26020884564615598386e1_f64 * t2146 * t9498 - t7921 + 0.13170898365871023197e1_f64 * t9018 - t7926 + 0.4336814094102599731e0_f64 * t2146 * t9503 + 0.17347256376410398924e1_f64 * t9031 - 0.17347256376410398924e1_f64 * t7931 * t9509 - 0.8673628188205199462e0_f64 * t2338 * t2351 - t7929;
    (t9503, t9508, t9509, t9514)
}
