//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 990/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk990(t1264: f64, t1410: f64, t157: f64, t2122: f64, t2127: f64, t2146: f64, t2152: f64, t2341: f64, t31944: f64, t31951: f64, t31955: f64, t31969: f64, t31972: f64, t33597: f64, t33606: f64, t33621: f64, t33624: f64, t33627: f64, t406: f64, t5332: f64, t7912: f64, t8004: f64, t8433: f64, t8993: f64) -> f64 {
    let t33628 = 0.52041769129231196772e1_f64 * t31944 + 0.34694512752820797848e1_f64 * t31951 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t8993 * t406 * t157 - 0.52041769129231196772e1_f64 * t31955 + 0.4336814094102599731e0_f64 * t2146 * t2152 * t33597 * t157 + 0.26341796731742046394e1_f64 * t31969 + 0.8673628188205199462e0_f64 * t7912 * t8433 - t33606 + 0.17347256376410398924e1_f64 * t31972 - 0.65854491829355115987e0_f64 * t2127 * t5332 - 0.26020884564615598386e1_f64 * t2146 * t8004 * t2341 * t1264 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t2122 * t1410 * t157 + t33621 - t33624 - t33627;
    t33628
}
