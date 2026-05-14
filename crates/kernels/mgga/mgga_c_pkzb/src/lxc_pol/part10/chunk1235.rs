//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1235/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1235<F: Float>(t23967: F, t23990: F, t2702: F, t1702: F, t9012: F, t6966: F, t8973: F, t2600: F, t7084: F, t1753: F, t8953: F, t3453: F, t5296: F, t3396: F, t568: F, t16379: F, t16381: F, t16389: F, t1692: F, t1733: F, t1734: F, t179: F, t19909: F, t19911: F, t19913: F, t19938: F, t19947: F, t19958: F, t19970: F, t19972: F, t19979: F, t2645: F, t50: F, t5279: F, t580: F, t581: F, t6896: F, t8962: F) -> (F, F, F, F, F, F) {
    let t23992 = t23967 / 2.0 + t23990 / 2.0;
    let t24015 = t2702 * t2702;
    let t24038 = t1702 * t9012;
    let t24040 = t6966 * t8973;
    let t24046 = t2600 * t7084;
    let t24050 = t8953 * t1753;
    let t24054 = t5296 * t3453;
    let t24064 = t3396 * t568;
    let t24069 = 35.0 / 18.0 * t19909 - 7.0 / 12.0 * t19911 - 7.0 / 24.0 * t19913 - t580 * t581 * t50 * t23992 / 48.0 + 0.80031500487063509015e-2 * t19938 + 7.0 / 72.0 * t24038 + 0.20007875121765877254e-2 * t24040 + 0.85748036236139473944e-3 * t1733 * t179 * t8962 * t1692 - 0.42874018118069736972e-3 * t2645 * t179 * t24046 - 0.12862205435420921092e-2 * t6896 * t179 * t24050 + 0.11337795902333997111e0 * t24054 + 0.30234122406223992295e0 * t19947 - 0.80031500487063509016e-2 * t19958 - 0.80031500487063509015e-2 * t19970 - 0.12004725073059526352e-1 * t19972 - 0.80031500487063509016e-2 * t19979 + 455.0 / 324.0 * t16379 - 35.0 / 216.0 * t16381 + 35.0 / 72.0 * t16389 - 0.85748036236139473944e-2 * t5279 * t179 * t24064 * t1734;
    (t23992, t24015, t24046, t24050, t24064, t24069)
}
