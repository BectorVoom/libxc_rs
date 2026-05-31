//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 460/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk460<F: Float>(t3572: F, t3573: F, t3577: F, t3581: F, t3585: F) -> F {
    let t3587 = t3572 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3573 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3577 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3581 - t3585 / F::cast_from(3.0_f64);
    t3587
}
