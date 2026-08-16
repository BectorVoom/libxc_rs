//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1198/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1198(t164: f64, t17034: f64, t1733: f64, t179: f64, t24189: f64, t24194: f64, t24251: f64, t24259: f64, t24269: f64, t2575: f64, t2600: f64, t2645: f64, t2646: f64, t29003: f64, t29248: f64, t29252: f64, t29254: f64, t29262: f64, t29264: f64, t29279: f64, t3401: f64, t5244: f64, t5279: f64, t568: f64, t8817: f64, t8914: f64, t8962: f64, t8971: f64, t9003: f64) -> f64 {
    let t29289 = 0.25724410870841842183e-2_f64 * t1733 * t179 * t2600 * t8817 - 0.51448821741683684367e-2_f64 * t5244 * t179 * t8914 * t2575 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t8962 * t2575 - 0.21437009059034868486e-3_f64 * t2645 * t179 * t29248 - 0.12004725073059526352e-1_f64 * t29252 + 0.24009450146119052705e-1_f64 * t29254 - 0.51448821741683684368e-2_f64 * t5244 * t179 * t29003 * t568 + 0.30011812682648815881e-2_f64 * t24251 - 0.17006693853500995666e-1_f64 * t24259 + 7.0_f64 / 144.0_f64 * t29262 + 7.0_f64 / 12.0_f64 * t29264 + 0.77173232612525526552e-1_f64 * t17034 * t179 * t2600 * t24189 - 0.25724410870841842184e-1_f64 * t5279 * t179 * t2600 * t24194 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t2646 * t164 * t3401 + 0.51448821741683684366e-2_f64 * t1733 * t179 * t2600 * t29279 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t8971 * t9003 + 0.48018900292238105408e-1_f64 * t24269;
    t29289
}
