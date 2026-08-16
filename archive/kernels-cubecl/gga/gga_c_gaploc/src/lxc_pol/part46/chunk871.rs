//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 871/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk871<F: Float>(t10215: F, t555: F, t188: F, t3377: F, t10485: F, t9333: F, t31139: F, t544: F, t986: F, t2386: F, t10525: F, t10526: F, t41965: F) -> (F, F, F, F, F) {
    let t42212 = t555 * t10215;
    let t42214 = t188 * t42212 * t3377;
    let t42216 = t10485 * t9333;
    let t42219 = t544 * t31139 * t986;
    let t42221 = F::cast_from(0.25025342966295298669e1_f64) * t42219 * t2386;
    let t42224 = F::cast_from(0.21450293971110256001e1_f64) * t10525 * t10526 * t41965;
    (t42212, t42214, t42216, t42221, t42224)
}
