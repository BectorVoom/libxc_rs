//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 733/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk733<F: Float>(t10660: F, t10664: F, t11352: F, t11355: F, t11358: F, t11361: F, t11382: F, t1648: F, t1815: F, t4624: F, t4652: F, t4664: F, t4667: F, t574: F) -> F {
    let t11385 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t11352 * t10664 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t11355 * t4624 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t4664 * t11358 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t11361 * t1648 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t4667 * t4652 + t1815 * t10660 / F::cast_from(4.0_f64) + t574 * t11382 / F::cast_from(2.0_f64);
    t11385
}
