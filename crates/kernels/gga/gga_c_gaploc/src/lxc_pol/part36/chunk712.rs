//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 712/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk712<F: Float>(t1020: F, t3113: F, t12555: F, t12558: F, t12566: F, t12569: F, t12580: F, t13087: F, t13088: F) -> F {
    let t13089 = t1020 * t3113;
    let t13091 = F::new(9.0) / F::new(256.0) * t12555;
    let t13092 = F::new(9.0) / F::new(8192.0) * t12558;
    let t13093 = F::new(3.0) / F::new(8192.0) * t12566;
    let t13094 = F::new(3.0) / F::new(256.0) * t12569;
    let t13095 = F::new(2.0) * t12580;
    let t13096 = t13087 + t13088 - t13089 / F::new(2.0) - t13091 - t13092 + t13093 + t13094 + t13095;
    t13096
}
