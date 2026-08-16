//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 753/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk753(t463: f64, t525: f64, t7932: f64, t2146: f64, t7307: f64, t7889: f64, t7931: f64, t8393: f64, t8398: f64, t8400: f64, t8403: f64, t8408: f64, t8411: f64, t8415: f64, t8420: f64, t8424: f64, t8428: f64, t8433: f64, t8437: f64) -> (f64, f64, f64) {
    let t8440 = t525 * t463;
    let t8441 = t7932 * t8440;
    let t8444 = 0.17347256376410398924e1_f64 * t7307 + 0.8673628188205199462e0_f64 * t2146 * t8393 - 0.8673628188205199462e0_f64 * t8398 + 0.4336814094102599731e0_f64 * t8400 * t8403 - 0.8673628188205199462e0_f64 * t8408 - t7889 - 0.26020884564615598386e1_f64 * t2146 * t8411 + 0.8673628188205199462e0_f64 * t2146 * t8415 + 0.8673628188205199462e0_f64 * t8420 - 0.8673628188205199462e0_f64 * t8424 + 0.4336814094102599731e0_f64 * t2146 * t8428 + 0.4336814094102599731e0_f64 * t2146 * t8433 + 0.8673628188205199462e0_f64 * t2146 * t8437 - 0.8673628188205199462e0_f64 * t7931 * t8441;
    (t8440, t8441, t8444)
}
