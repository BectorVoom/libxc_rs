//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 887/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk887<F: Float>(t2331: F, t943: F, t7908: F, t8998: F, t33489: F, t7963: F, t7965: F, t4210: F, t7942: F, t315: F, t5386: F, t610: F, t1264: F, t1410: F, t157: F, t2122: F, t2127: F, t2146: F, t2152: F, t2341: F, t31944: F, t31951: F, t31955: F, t31969: F, t31972: F, t406: F, t5332: F, t7912: F, t8004: F, t8433: F, t8993: F) -> (F, F) {
    let t33597 = t2331 * t943;
    let t33606 = 0.34694512752820797848e1 * t8998 * t7908;
    let t33621 = 0.17347256376410398924e1 * t7963 * t33489 * t7965;
    let t33624 = 0.17347256376410398924e1 * t7942 * t33489 * t4210;
    let t33627 = 0.26341796731742046394e1 * t315 * t610 * t5386;
    let t33628 = 0.52041769129231196772e1 * t31944 + 0.34694512752820797848e1 * t31951 + 0.8673628188205199462e0 * t2146 * t2152 * t8993 * t406 * t157 - 0.52041769129231196772e1 * t31955 + 0.4336814094102599731e0 * t2146 * t2152 * t33597 * t157 + 0.26341796731742046394e1 * t31969 + 0.8673628188205199462e0 * t7912 * t8433 - t33606 + 0.17347256376410398924e1 * t31972 - 0.65854491829355115987e0 * t2127 * t5332 - 0.26020884564615598386e1 * t2146 * t8004 * t2341 * t1264 + 0.8673628188205199462e0 * t2146 * t2152 * t2122 * t1410 * t157 + t33621 - t33624 - t33627;
    (t33597, t33628)
}
