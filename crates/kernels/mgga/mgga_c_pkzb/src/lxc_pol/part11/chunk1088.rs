//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1088/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1088<F: Float>(t10621: F, t164: F, t600: F, t10655: F, t5257: F, t10651: F, t16399: F, t10558: F, t1702: F, t10562: F, t16369: F, t1020: F, t2639: F, t17034: F, t1733: F, t179: F, t24189: F, t24194: F, t24251: F, t24259: F, t24269: F, t2575: F, t2600: F, t2645: F, t2646: F, t29003: F, t3401: F, t5244: F, t5279: F, t568: F, t8817: F, t8914: F, t8962: F, t8971: F, t9003: F) -> (F, F, F) {
    let t29248 = t10621 * t600 * t164;
    let t29252 = t5257 * t10655;
    let t29254 = t16399 * t10651;
    let t29262 = t1702 * t10558;
    let t29264 = t16369 * t10562;
    let t29279 = t1020 * t2639;
    let t29289 = 0.25724410870841842183e-2 * t1733 * t179 * t2600 * t8817 - 0.51448821741683684367e-2 * t5244 * t179 * t8914 * t2575 + 0.25724410870841842183e-2 * t1733 * t179 * t8962 * t2575 - 0.21437009059034868486e-3 * t2645 * t179 * t29248 - 0.12004725073059526352e-1 * t29252 + 0.24009450146119052705e-1 * t29254 - 0.51448821741683684368e-2 * t5244 * t179 * t29003 * t568 + 0.30011812682648815881e-2 * t24251 - 0.17006693853500995666e-1 * t24259 + 7.0 / 144.0 * t29262 + 7.0 / 12.0 * t29264 + 0.77173232612525526552e-1 * t17034 * t179 * t2600 * t24189 - 0.25724410870841842184e-1 * t5279 * t179 * t2600 * t24194 - 0.12862205435420921092e-1 * t5279 * t179 * t2646 * t164 * t3401 + 0.51448821741683684366e-2 * t1733 * t179 * t2600 * t29279 + 0.25724410870841842183e-2 * t1733 * t179 * t8971 * t9003 + 0.48018900292238105408e-1 * t24269;
    (t29248, t29279, t29289)
}
