//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1237/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1237<F: Float>(t16425: F, t1719: F, t3410: F, t8953: F, t2593: F, t7084: F, t17043: F, t8978: F, t6892: F, t8921: F, t164: F, t16403: F, t16407: F, t16440: F, t1733: F, t1753: F, t179: F, t20065: F, t24105: F, t2592: F, t3396: F, t600: F, t6896: F, t8817: F) -> (F, F, F, F) {
    let t24110 = t3410 * t16425 * t1719;
    let t24114 = t8953 * t1719;
    let t24131 = t2593 * t7084;
    let t24135 = t17043 * t8978;
    let t24137 = t6892 * t8921;
    let t24139 = -0.12862205435420921092e-2 * t6896 * t179 * t24105 + 0.51448821741683684368e-2 * t20065 * t179 * t24110 - 0.77173232612525526552e-2 * t6896 * t179 * t24114 + 0.11337795902333997111e-1 * t16403 + 0.75585306015559980738e-1 * t16407 - 0.56688979511669985553e-2 * t16440 + 0.17149607247227894789e-2 * t1733 * t179 * t8817 * t600 * t164 + 0.85748036236139473944e-3 * t1733 * t179 * t3396 * t1753 * t164 + 0.85748036236139473944e-3 * t2592 * t179 * t24131 + 0.40015750243531754508e-1 * t24135 - 0.80031500487063509015e-2 * t24137;
    (t24110, t24114, t24131, t24139)
}
