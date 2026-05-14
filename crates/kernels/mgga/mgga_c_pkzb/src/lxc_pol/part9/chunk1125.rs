//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1125/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1125<F: Float>(t17850: F, t17864: F, t17869: F, t17872: F, t2037: F, t21435: F, t21448: F, t21452: F, t21455: F, t21456: F, t21457: F, t21462: F, t21463: F, t21468: F, t21469: F, t21485: F, t2899: F, t2922: F, t2925: F, t2933: F, t302: F, t6022: F, t7653: F, t7658: F, t7736: F, t7737: F) -> (F,) {
    let t21489 = -0.64311027177104605458e-3 * t2922 * t302 * t7653 * t7658 - 0.21722835846488666732e-1 * t2037 * t21448 * t2925 - 0.91464571985215438873e-2 * t21452 + 0.30011812682648815881e-2 * t21455 * t302 * t21456 * t21457 + 0.51448821741683684368e-2 * t21462 * t302 * t21456 * t21463 - 0.77173232612525526552e-2 * t21468 * t302 * t21456 * t21469 + 0.12862205435420921092e-2 * t2899 * t302 * t7653 * t6022 + 0.38586616306262763275e-2 * t7736 * t302 * t21435 * t7737 + 0.25724410870841842183e-2 * t17850 + 0.57165357490759649295e-3 * t17869 - 0.85748036236139473944e-3 * t17872 + 0.91464571985215438873e-2 * t21485 - 0.43445671692977333464e-1 * t17864 * t2933;
    (t21489,)
}
