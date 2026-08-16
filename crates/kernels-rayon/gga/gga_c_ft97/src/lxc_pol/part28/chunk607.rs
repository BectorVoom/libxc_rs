//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 607/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk607(t25779: f64, t73: f64, t25658: f64, t5513: f64, t25643: f64, t72: f64, t5579: f64, t1603: f64, t22522: f64, t22534: f64, t22568: f64, t22619: f64, t22761: f64, t22775: f64, t22796: f64, t22804: f64, t22826: f64, t25734: f64, t25746: f64, t25750: f64, t25753: f64, t25756: f64, t25760: f64, t25768: f64, t25771: f64, t25775: f64, t3034: f64, t3038: f64, t5569: f64, t5570: f64, t5611: f64, t6441: f64) -> (f64, f64, f64, f64) {
    let t25780 = t73 * t25779;
    let t25784 = t5513 * t25658;
    let t25787 = t72 * t25643;
    let t25788 = t5579 * t25787;
    let t25791 = 0.23254900946437792e-1_f64 * t22826 * t3034 + 0.46509801892875584e-2_f64 * t25734 * t3038 - 0.12768721675925925926e-1_f64 * t22775 + 0.17024962234567901235e-1_f64 * t5611 * t25746 - 0.21281202793209876543e-2_f64 * t25750 - 0.51789017496114396277e-5_f64 * t25753 * t25756 + 0.12768721675925925926e-1_f64 * t22522 * t5570 * t25760 - 0.59387071557258112888e-3_f64 * t5569 * t22568 * t6441 + 0.7423383944657264111e-4_f64 * t25768 + 0.52801466802079540469e-5_f64 * t22796 * t25771 - 0.14836531933660919214e-4_f64 * t22534 * t25775 - 0.44540303667943584666e-3_f64 * t22619 * t25780 - 0.21281202793209876543e-2_f64 * t22804 - 0.23254900946437792e-1_f64 * t1603 * t25784 - 0.11491849508333333333e0_f64 * t22761 * t25788;
    (t25780, t25787, t25788, t25791)
}
