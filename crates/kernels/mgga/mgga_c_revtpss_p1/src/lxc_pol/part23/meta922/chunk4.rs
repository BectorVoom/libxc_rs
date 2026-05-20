//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2983/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2983<F: Float>(t1032: F, t1040: F, t23959: F, t1042: F, t1047: F, t1063: F, t11860: F, t15906: F, t16199: F, t19450: F, t19722: F, t19748: F, t19971: F, t22671: F, t23830: F, t3117: F, t3127: F, t42121: F, t42690: F, t43105: F, t43207: F, t4872: F, t4910: F, t54316: F, t65589: F, t65596: F, t65598: F, t65610: F, t65618: F, t65627: F, t65630: F, t65637: F, t65650: F, t66431: F, t78496: F, t78790: F, t78812: F, t999: F) -> F {
    let t79038 = t23959 * t1032 * t1040;
    let t79049 = F::cast_from(0.30488190661738479624e-2_f64) * t65589 + F::cast_from(0.19055119163586549765e-3_f64) * t65596 + F::cast_from(0.85748036236139473944e-3_f64) * t65598 + F::cast_from(0.95275595817932748825e-3_f64) * t65610 - F::cast_from(0.42874018118069736972e-3_f64) * t65618 + F::cast_from(0.85748036236139473944e-3_f64) * t65627 - F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t1042 * t4872 * t22671 * t999 - F::cast_from(0.57165357490759649296e-3_f64) * t65630 - F::cast_from(0.77173232612525526552e-2_f64) * t54316 * t3117 * t78812 * t19748 + F::cast_from(0.12862205435420921092e-2_f64) * t43105 * t3117 * t78496 * t11860 - F::cast_from(0.38586616306262763276e-2_f64) * t15906 * t3117 * t19450 * t19971 + F::cast_from(0.64311027177104605458e-3_f64) * t66431 * t19722 - F::cast_from(0.21437009059034868486e-3_f64) * t42690 * t3117 * t78496 * t4910 + F::cast_from(0.21437009059034868486e-3_f64) * t79038 * t1047 - F::cast_from(0.28582678745379824648e-3_f64) * t65637 + F::cast_from(0.47637797908966374413e-3_f64) * t65650 - t42121 - F::cast_from(0.68598428988911579157e-2_f64) * t43207 * t23830 - F::cast_from(0.42874018118069736972e-2_f64) * t1063 * t1042 * t16199 * t78790;
    t79049
}
