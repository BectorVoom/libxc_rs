//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 963/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk963<F: Float>(t1411: F, t14236: F, t14201: F, t14206: F, t14211: F, t14216: F, t14218: F, t14220: F, t14224: F, t14226: F, t14228: F, t14230: F, t14232: F) -> (F, F) {
    let t14237 = t1411 * t14236;
    let t14239 = F::new(0.2653111111111111111e-1) * t14201 - F::new(0.49745833333333333332e-2) * t14206 + F::new(0.73697530864197530862e-3) * t14211 + F::new(0.44218518518518518518e-2) * t14216 + F::new(0.33163888888888888887e-2) * t14218 - F::new(0.99491666666666666664e-2) * t14220 + F::new(0.16581944444444444444e-2) * t14224 - F::new(0.11054629629629629629e-2) * t14226 - F::new(0.17687407407407407407e-1) * t14228 - F::new(0.66327777777777777776e-2) * t14230 - F::new(0.17687407407407407407e-1) * t14232 + F::new(0.99491666666666666664e-2) * t14237;
    (t14237, t14239)
}
