//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1151/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1151<F: Float>(t1634: F, t164: F, t16438: F, t16440: F, t16453: F, t16459: F, t16467: F, t16474: F, t1733: F, t179: F, t19974: F, t19995: F, t19997: F, t20002: F, t20004: F, t20011: F, t20017: F, t20019: F, t2592: F, t2646: F, t5279: F, t568: F, t6970: F) -> F {
    let t20032 = -F::new(0.60023625365297631762e-2) * t16438 - F::new(0.17006693853500995666e-1) * t16440 + F::new(0.60023625365297631762e-2) * t19995 + F::new(0.25724410870841842183e-2) * t1733 * t179 * t19997 * t568 - F::new(0.24009450146119052704e-1) * t20002 + F::new(0.30011812682648815881e-2) * t20004 - F::new(0.12862205435420921092e-1) * t5279 * t179 * t6970 * t1634 + F::new(0.18007087609589289528e-1) * t20011 + F::new(0.25724410870841842183e-2) * t1733 * t179 * t19974 * t164 - F::new(0.12004725073059526352e-1) * t20017 + F::new(0.38586616306262763275e-2) * t2592 * t179 * t20019 + F::new(0.30011812682648815881e-2) * t16453 + F::new(0.10003937560882938627e-2) * t16459 - F::new(0.17006693853500995666e-1) * t16467 + F::new(0.10003937560882938627e-2) * t16474 - F::new(0.12862205435420921092e-1) * t5279 * t179 * t2646 * t164 * t1634;
    t20032
}
