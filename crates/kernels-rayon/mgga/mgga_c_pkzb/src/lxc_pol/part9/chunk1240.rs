//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1240/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1240(t1124: f64, t300: f64, t179: f64, t2739: f64, t299: f64, t5672: f64, t771: f64, t7765: f64, t17765: f64, t2009: f64, t20743: f64, t2082: f64, t2104: f64, t2106: f64, t2107: f64, t21417: f64, t21518: f64, t2899: f64, t2922: f64, t2923: f64, t2952: f64, t302: f64, t5703: f64, t5728: f64, t5965: f64, t7648: f64, t7664: f64, t7700: f64, t7701: f64, t7742: f64, t7769: f64, t7787: f64, t780: f64) -> (f64, f64) {
    let t21686 = t300 * t1124;
    let t21714 = t299 * t179 * t5672 * t2739;
    let t21715 = 0.28582678745379824648e-3_f64 * t21714;
    let t21718 = t771 * t7765;
    let t21724 = 0.77173232612525526551e-2_f64 * t2104 * t300 * t7787 * t2107 - 0.51448821741683684367e-2_f64 * t2899 * t21417 * t7769 + 0.64311027177104605458e-3_f64 * t7664 * t21686 * t5728 * t2009 * t2923 + 0.12862205435420921092e-2_f64 * t2922 * t7700 * t5703 * t2106 + 0.12862205435420921092e-2_f64 * t2922 * t7700 * t7701 * t7648 + 0.77173232612525526551e-2_f64 * t7742 * t7700 * t17765 * t5965 - 0.12862205435420921092e-2_f64 * t7664 * t7700 * t17765 * t2106 - 0.42874018118069736972e-3_f64 * t299 * t179 * t780 * t20743 + t21715 - 0.43445671692977333464e-1_f64 * t2082 * t2952 + 0.91464571985215438873e-2_f64 * t21718 - 0.64311027177104605458e-3_f64 * t2922 * t302 * t21518 * t2923;
    (t21686, t21724)
}
