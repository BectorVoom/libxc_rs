//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 872/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk872<F: Float>(t11352: F, t16037: F, t1815: F, t22599: F, t2372: F, t28393: F, t28414: F, t28546: F, t28568: F, t4664: F, t574: F, t6774: F, t8504: F, t8522: F) -> F {
    let t28571 = F::new(3.0) / F::new(16.0) * t11352 * t28414 - F::new(3.0) / F::new(8.0) * t16037 * t8504 - F::new(3.0) / F::new(8.0) * t4664 * t28546 + F::new(3.0) / F::new(4.0) * t22599 * t2372 + F::new(3.0) / F::new(4.0) * t6774 * t8522 + t1815 * t28393 / F::new(4.0) + t574 * t28568 / F::new(2.0);
    t28571
}
