//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1151/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1151(t1634: f64, t164: f64, t16438: f64, t16440: f64, t16453: f64, t16459: f64, t16467: f64, t16474: f64, t1733: f64, t179: f64, t19974: f64, t19995: f64, t19997: f64, t20002: f64, t20004: f64, t20011: f64, t20017: f64, t20019: f64, t2592: f64, t2646: f64, t5279: f64, t568: f64, t6970: f64) -> f64 {
    let t20032 = -0.60023625365297631762e-2_f64 * t16438 - 0.17006693853500995666e-1_f64 * t16440 + 0.60023625365297631762e-2_f64 * t19995 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t19997 * t568 - 0.24009450146119052704e-1_f64 * t20002 + 0.30011812682648815881e-2_f64 * t20004 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t6970 * t1634 + 0.18007087609589289528e-1_f64 * t20011 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t19974 * t164 - 0.12004725073059526352e-1_f64 * t20017 + 0.38586616306262763275e-2_f64 * t2592 * t179 * t20019 + 0.30011812682648815881e-2_f64 * t16453 + 0.10003937560882938627e-2_f64 * t16459 - 0.17006693853500995666e-1_f64 * t16467 + 0.10003937560882938627e-2_f64 * t16474 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t2646 * t164 * t1634;
    t20032
}
