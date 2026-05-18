//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1168/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1168<F: Float>(t13945: F, t650: F, t43295: F, t43298: F, t43300: F, t43302: F, t43304: F, t47749: F, t47752: F, t47755: F, t47758: F, t47764: F) -> F {
    let t47766 = F::new(0.10254034973522965712e-1) * t650 * t13945;
    let t47767 = t43295 - F::new(0.46143157380853345701e-1) * t43298 + t43300 - F::new(0.53833683610995569986e-1) * t43302 - F::new(0.53833683610995569986e-1) * t47749 + F::new(0.10254034973522965712e-1) * t43304 + F::new(0.76905262301422242837e-2) * t47752 + F::new(0.76905262301422242837e-2) * t47755 + F::new(0.76905262301422242837e-2) * t47758 + t47764 - t47766;
    t47767
}
