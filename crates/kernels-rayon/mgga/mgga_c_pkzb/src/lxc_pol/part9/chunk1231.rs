//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1231/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1231(t18009: f64, t21454: f64, t5726: f64, t5955: f64, t5984: f64, t7637: f64, t17850: f64, t17864: f64, t17869: f64, t17872: f64, t2037: f64, t21435: f64, t21448: f64, t21452: f64, t21455: f64, t21456: f64, t21457: f64, t21462: f64, t21463: f64, t2899: f64, t2922: f64, t2925: f64, t2933: f64, t302: f64, t6022: f64, t7653: f64, t7658: f64, t7736: f64, t7737: f64) -> (f64, f64) {
    let t21468 = t18009 * t21454;
    let t21469 = t5955 * t5726;
    let t21485 = t5984 * t7637;
    let t21489 = -0.64311027177104605458e-3_f64 * t2922 * t302 * t7653 * t7658 - 0.21722835846488666732e-1_f64 * t2037 * t21448 * t2925 - 0.91464571985215438873e-2_f64 * t21452 + 0.30011812682648815881e-2_f64 * t21455 * t302 * t21456 * t21457 + 0.51448821741683684368e-2_f64 * t21462 * t302 * t21456 * t21463 - 0.77173232612525526552e-2_f64 * t21468 * t302 * t21456 * t21469 + 0.12862205435420921092e-2_f64 * t2899 * t302 * t7653 * t6022 + 0.38586616306262763275e-2_f64 * t7736 * t302 * t21435 * t7737 + 0.25724410870841842183e-2_f64 * t17850 + 0.57165357490759649295e-3_f64 * t17869 - 0.85748036236139473944e-3_f64 * t17872 + 0.91464571985215438873e-2_f64 * t21485 - 0.43445671692977333464e-1_f64 * t17864 * t2933;
    (t21469, t21489)
}
