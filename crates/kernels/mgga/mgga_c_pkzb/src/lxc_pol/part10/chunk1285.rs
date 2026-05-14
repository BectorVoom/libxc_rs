//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1285/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1285<F: Float>(t1123: F, t300: F, t5633: F, t17938: F, t3638: F, t1066: F, t1885: F, t2104: F, t21455: F, t21462: F, t21468: F, t21611: F, t21614: F, t21617: F, t21620: F, t21623: F, t21626: F, t21633: F, t21637: F, t21640: F, t25147: F, t302: F, t6022: F, t761: F, t7737: F, t7742: F, t7743: F, t9282: F) -> (F, F) {
    let t25357 = t300 * t5633 * t1123;
    let t25363 = t3638 * t17938;
    let t25389 = -0.10289764348336736873e-1 * t2104 * t25357 * t761 * t1066 * t1885 + 0.51448821741683684368e-2 * t21462 * t302 * t25363 * t25147 - 0.77173232612525526552e-2 * t21468 * t302 * t25363 * t7737 - 0.12862205435420921092e-2 * t7742 * t302 * t9282 * t6022 + 0.30011812682648815881e-2 * t21455 * t302 * t25363 * t7743 + 0.11433071498151929859e-2 * t21611 + 0.57165357490759649296e-3 * t21614 + 0.17149607247227894789e-2 * t21617 - 0.17149607247227894789e-2 * t21620 + 0.3811023832717309953e-3 * t21623 + 0.19055119163586549765e-3 * t21626 - 0.11433071498151929859e-2 * t21633 - 0.57165357490759649296e-3 * t21637 - 0.11433071498151929859e-2 * t21640;
    (t25363, t25389)
}
