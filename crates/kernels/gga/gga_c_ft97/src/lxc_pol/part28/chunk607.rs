//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 607/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk607<F: Float>(t25779: F, t73: F, t25658: F, t5513: F, t25643: F, t72: F, t5579: F, t1603: F, t22522: F, t22534: F, t22568: F, t22619: F, t22761: F, t22775: F, t22796: F, t22804: F, t22826: F, t25734: F, t25746: F, t25750: F, t25753: F, t25756: F, t25760: F, t25768: F, t25771: F, t25775: F, t3034: F, t3038: F, t5569: F, t5570: F, t5611: F, t6441: F) -> (F, F, F, F) {
    let t25780 = t73 * t25779;
    let t25784 = t5513 * t25658;
    let t25787 = t72 * t25643;
    let t25788 = t5579 * t25787;
    let t25791 = F::new(0.23254900946437792e-1) * t22826 * t3034 + F::new(0.46509801892875584e-2) * t25734 * t3038 - F::new(0.12768721675925925926e-1) * t22775 + F::new(0.17024962234567901235e-1) * t5611 * t25746 - F::new(0.21281202793209876543e-2) * t25750 - F::new(0.51789017496114396277e-5) * t25753 * t25756 + F::new(0.12768721675925925926e-1) * t22522 * t5570 * t25760 - F::new(0.59387071557258112888e-3) * t5569 * t22568 * t6441 + F::new(0.7423383944657264111e-4) * t25768 + F::new(0.52801466802079540469e-5) * t22796 * t25771 - F::new(0.14836531933660919214e-4) * t22534 * t25775 - F::new(0.44540303667943584666e-3) * t22619 * t25780 - F::new(0.21281202793209876543e-2) * t22804 - F::new(0.23254900946437792e-1) * t1603 * t25784 - F::new(0.11491849508333333333e0) * t22761 * t25788;
    (t25780, t25787, t25788, t25791)
}
