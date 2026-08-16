//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1149/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1149(t5257: f64, t6976: f64, t16363: f64, t16370: f64, t16379: f64, t16381: f64, t16389: f64, t16400: f64, t16403: f64, t16407: f64, t16409: f64, t16417: f64, t1721: f64, t179: f64, t19958: f64, t19961: f64, t19966: f64, t19970: f64, t19972: f64, t19974: f64, t2592: f64, t2645: f64, t5244: f64) -> f64 {
    let t19979 = t5257 * t6976;
    let t19991 = -0.12004725073059526352e-1_f64 * t19958 + 0.12862205435420921092e-2_f64 * t2592 * t179 * t19961 - 0.64311027177104605458e-3_f64 * t2645 * t179 * t19966 - 0.12004725073059526352e-1_f64 * t19970 - 0.18007087609589289528e-1_f64 * t19972 - 0.51448821741683684367e-2_f64 * t5244 * t179 * t19974 * t1721 - 0.12004725073059526352e-1_f64 * t19979 + 7.0_f64 / 144.0_f64 * t16363 + 7.0_f64 / 12.0_f64 * t16370 + 455.0_f64 / 216.0_f64 * t16379 - 35.0_f64 / 72.0_f64 * t16381 + 35.0_f64 / 24.0_f64 * t16389 + 0.24009450146119052705e-1_f64 * t16400 + 0.34013387707001991333e-1_f64 * t16403 + 0.11337795902333997111e0_f64 * t16407 - 0.60023625365297631762e-1_f64 * t16409 + 0.60023625365297631762e-2_f64 * t16417;
    t19991
}
