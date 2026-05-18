//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1240/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1240<F: Float>(t1124: F, t300: F, t179: F, t2739: F, t299: F, t5672: F, t771: F, t7765: F, t17765: F, t2009: F, t20743: F, t2082: F, t2104: F, t2106: F, t2107: F, t21417: F, t21518: F, t2899: F, t2922: F, t2923: F, t2952: F, t302: F, t5703: F, t5728: F, t5965: F, t7648: F, t7664: F, t7700: F, t7701: F, t7742: F, t7769: F, t7787: F, t780: F) -> (F, F) {
    let t21686 = t300 * t1124;
    let t21714 = t299 * t179 * t5672 * t2739;
    let t21715 = F::new(0.28582678745379824648e-3) * t21714;
    let t21718 = t771 * t7765;
    let t21724 = F::new(0.77173232612525526551e-2) * t2104 * t300 * t7787 * t2107 - F::new(0.51448821741683684367e-2) * t2899 * t21417 * t7769 + F::new(0.64311027177104605458e-3) * t7664 * t21686 * t5728 * t2009 * t2923 + F::new(0.12862205435420921092e-2) * t2922 * t7700 * t5703 * t2106 + F::new(0.12862205435420921092e-2) * t2922 * t7700 * t7701 * t7648 + F::new(0.77173232612525526551e-2) * t7742 * t7700 * t17765 * t5965 - F::new(0.12862205435420921092e-2) * t7664 * t7700 * t17765 * t2106 - F::new(0.42874018118069736972e-3) * t299 * t179 * t780 * t20743 + t21715 - F::new(0.43445671692977333464e-1) * t2082 * t2952 + F::new(0.91464571985215438873e-2) * t21718 - F::new(0.64311027177104605458e-3) * t2922 * t302 * t21518 * t2923;
    (t21686, t21724)
}
