//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 839/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk839<F: Float>(t10502: F, t16640: F, t16658: F, t17078: F, t22353: F, t22355: F, t28244: F, t28250: F, t28253: F, t28259: F, t28262: F, t28271: F, t4823: F, t8852: F) -> F {
    let t28273 = F::new(0.55273148148148148145e-2) * t28244 + F::new(0.55273148148148148145e-2) * t22353 + F::new(0.33163888888888888887e-2) * t22355 + F::new(0.99491666666666666664e-2) * t28250 + F::new(0.8290972222222222222e-2) * t28253 + t10502 - F::new(0.16581944444444444444e-2) * t16640 + F::new(0.99491666666666666664e-2) * t28259 - F::new(0.223494e0) * t4823 * t28262 + F::new(0.223494e0) * t17078 * t8852 - F::new(0.11054629629629629629e-2) * t16658 - F::new(0.49745833333333333332e-2) * t28271;
    t28273
}
