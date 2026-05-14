//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 509/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk509<F: Float>(t1548: F, t775: F, t800: F, t4365: F, t837: F, t4364: F, t125: F, t1544: F, t2747: F, t1549: F, t2703: F, t124: F, t4343: F, t2749: F, t2488: F, t2653: F, t2666: F, t2678: F, t2691: F, t2695: F, t2702: F, t2716: F, t2730: F, t2739: F, t2745: F, t799: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4442 = t800 * t1548 * t775;
    let t4446 = t4365 * t837;
    let t4447 = t4364 * t4446;
    let t4450 = t125 * t1544;
    let t4451 = t4450 * t837;
    let t4452 = t2747 * t4451;
    let t4455 = t2703 * t1549;
    let t4457 = t124 * t4343;
    let t4458 = t800 * t4457;
    let t4461 = t4365 * t2749;
    let t4462 = t2747 * t4461;
    let t4468 = t2716 - 0.12705000702321332056e-4 * t2488 + t2730 * t4442 / 16.0 + t2691 + 0.28582678745379824648e-4 * t2695 + t2702 - t2739 - 0.21437009059034868486e-3 * t2745 * t4447 + 0.85748036236139473944e-3 * t2745 * t4452 + 7.0 / 144.0 * t4455 - t799 * t4458 / 48.0 + 0.85748036236139473944e-3 * t2745 * t4462 + 0.40015750243531754508e-2 * t2653 + 0.71456696863449561619e-5 * t2666 - 0.50820002809285328224e-4 * t2678;
    (t4446, t4447, t4450, t4451, t4452, t4458, t4461, t4462, t4468)
}
