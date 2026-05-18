//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1149/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1149<F: Float>(t5257: F, t6976: F, t16363: F, t16370: F, t16379: F, t16381: F, t16389: F, t16400: F, t16403: F, t16407: F, t16409: F, t16417: F, t1721: F, t179: F, t19958: F, t19961: F, t19966: F, t19970: F, t19972: F, t19974: F, t2592: F, t2645: F, t5244: F) -> F {
    let t19979 = t5257 * t6976;
    let t19991 = -F::new(0.12004725073059526352e-1) * t19958 + F::new(0.12862205435420921092e-2) * t2592 * t179 * t19961 - F::new(0.64311027177104605458e-3) * t2645 * t179 * t19966 - F::new(0.12004725073059526352e-1) * t19970 - F::new(0.18007087609589289528e-1) * t19972 - F::new(0.51448821741683684367e-2) * t5244 * t179 * t19974 * t1721 - F::new(0.12004725073059526352e-1) * t19979 + F::new(7.0) / F::new(144.0) * t16363 + F::new(7.0) / F::new(12.0) * t16370 + F::new(455.0) / F::new(216.0) * t16379 - F::new(35.0) / F::new(72.0) * t16381 + F::new(35.0) / F::new(24.0) * t16389 + F::new(0.24009450146119052705e-1) * t16400 + F::new(0.34013387707001991333e-1) * t16403 + F::new(0.11337795902333997111e0) * t16407 - F::new(0.60023625365297631762e-1) * t16409 + F::new(0.60023625365297631762e-2) * t16417;
    t19991
}
