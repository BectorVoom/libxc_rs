//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 889/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk889<F: Float>(t13312: F, t3482: F, t12819: F, t12822: F, t12834: F, t12836: F, t12838: F, t12842: F, t13291: F, t13297: F, t13302: F, t13307: F, t13309: F) -> (F, F) {
    let t13313 = t3482 * t13312;
    let t13315 = F::new(0.49745833333333333332e-2) * t12819 + F::new(0.49745833333333333332e-2) * t12822 + F::new(0.73697530864197530861e-2) * t12834 + F::new(0.66327777777777777776e-2) * t12836 + F::new(0.33163888888888888887e-2) * t12838 + F::new(0.55273148148148148145e-2) * t12842 - F::new(0.24872916666666666666e-2) * t13291 - F::new(0.66327777777777777775e-2) * t13297 + F::new(0.99491666666666666664e-2) * t13302 - F::new(0.99491666666666666664e-2) * t13307 + F::new(0.66327777777777777776e-2) * t13309 - F::new(0.17687407407407407407e-1) * t13313;
    (t13313, t13315)
}
