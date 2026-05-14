//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1056/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1056<F: Float>(t6866: F, t6892: F, t1721: F, t600: F, t7084: F, t1719: F, t2639: F, t164: F, t5257: F, t6877: F, t6904: F, t2575: F, t6976: F, t16363: F, t16370: F, t16379: F, t16381: F, t16389: F, t16400: F, t16403: F, t16407: F, t16409: F, t16417: F, t179: F, t2592: F, t2645: F, t5244: F) -> (F, F, F, F, F) {
    let t19958 = t6892 * t6866;
    let t19961 = t7084 * t1721 * t600;
    let t19965 = t2639 * t1719;
    let t19966 = t19965 * t164;
    let t19970 = t5257 * t6877;
    let t19972 = t6892 * t6904;
    let t19974 = t2575 * t1719;
    let t19979 = t5257 * t6976;
    let t19991 = -0.12004725073059526352e-1 * t19958 + 0.12862205435420921092e-2 * t2592 * t179 * t19961 - 0.64311027177104605458e-3 * t2645 * t179 * t19966 - 0.12004725073059526352e-1 * t19970 - 0.18007087609589289528e-1 * t19972 - 0.51448821741683684367e-2 * t5244 * t179 * t19974 * t1721 - 0.12004725073059526352e-1 * t19979 + 7.0 / 144.0 * t16363 + 7.0 / 12.0 * t16370 + 455.0 / 216.0 * t16379 - 35.0 / 72.0 * t16381 + 35.0 / 24.0 * t16389 + 0.24009450146119052705e-1 * t16400 + 0.34013387707001991333e-1 * t16403 + 0.11337795902333997111e0 * t16407 - 0.60023625365297631762e-1 * t16409 + 0.60023625365297631762e-2 * t16417;
    (t19961, t19965, t19966, t19974, t19991)
}
