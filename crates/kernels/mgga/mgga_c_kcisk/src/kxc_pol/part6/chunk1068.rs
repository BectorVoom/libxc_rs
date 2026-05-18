//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1068/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1068<F: Float>(t14784: F, t14785: F, t19543: F, t30592: F, t30595: F, t30599: F, t30603: F, t30613: F, t30617: F, t30623: F, t30626: F, t30629: F, t30632: F, t30635: F) -> F {
    let t31556 = F::new(0.94674375e0) * t30613 - t14784 - t14785 - F::new(0.34731666666666666667e0) * t19543 + F::new(0.264729375e1) * t30617 + F::new(0.20659e1) * t30595 - F::new(0.309885e1) * t30599 - F::new(0.57386111111111111112e0) * t30592 - F::new(0.516475e0) * t30603 - F::new(0.157790625e0) * t30623 - F::new(0.46308888888888888889e-1) * t30626 - F::new(0.104195e0) * t30629 + F::new(0.20839e0) * t30632 - F::new(0.62517e0) * t30635;
    t31556
}
