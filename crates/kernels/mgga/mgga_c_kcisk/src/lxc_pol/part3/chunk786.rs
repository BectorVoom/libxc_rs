//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 786/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk786<F: Float>(t240: F, t4753: F, t10549: F, t10683: F, t10707: F, t10709: F, t10712: F, t10718: F, t10752: F, t10760: F, t10773: F, t1686: F, t1987: F, t4783: F, t4791: F, t5423: F) -> F {
    let t12131 = t240 * t4753;
    let t12142 = -F::new(0.58482233974552040708e0) * t1987 * t10683 - F::new(0.17544670192365612213e1) * t12131 * t1686 - F::new(0.17544670192365612213e1) * t5423 * t4783 - F::new(0.51947267698127589899e2) * t5423 * t4791 - F::new(0.51947267698127589897e2) * t1987 * t10549 + F::new(0.19751789702565206229e-1) * t240 * t10773 + t10707 + t10709 + t10712 - t10718 + t10752 + t10760;
    t12142
}
