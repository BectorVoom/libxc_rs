//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1001/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1001<F: Float>(t10767: F, t179: F, t780: F, t2946: F, t3515: F, t758: F, t10934: F, t10938: F, t10945: F, t10982: F, t10986: F, t10990: F, t10995: F, t2887: F, t2899: F, t2945: F, t299: F, t5591: F, t5614: F, t5725: F, t5933: F, t5954: F, t757: F, t7582: F, t7621: F, t7756: F, t9308: F) -> (F, F, F, F) {
    let t10999 = t179 * t780 * t10767;
    let t11004 = t2946 * t3515;
    let t11005 = t758 * t11004;
    let t11008 = -F::new(0.14291339372689912324e-3) * t7582 + t5591 + t5614 - F::new(0.51448821741683684368e-2) * t299 * t10934 + t2887 * t10938 / F::new(16.0) + t7621 / F::new(144.0) + F::new(0.21437009059034868486e-3) * t5933 * t10945 + F::new(0.21437009059034868486e-3) * t757 * t10982 + F::new(0.12862205435420921092e-2) * t5954 * t10986 - F::new(0.12862205435420921092e-2) * t5725 * t10990 + F::new(0.12862205435420921092e-2) * t2899 * t10995 - F::new(0.42874018118069736972e-3) * t299 * t10999 - F::new(0.17149607247227894789e-2) * t9308 + F::new(0.28582678745379824648e-3) * t7756 + F::new(0.38586616306262763276e-2) * t2945 * t11005;
    (t10999, t11004, t11005, t11008)
}
