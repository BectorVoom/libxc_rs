//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 987/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk987<F: Float>(t2508: F, t2717: F, t3722: F, t12305: F, t954: F, t169: F, t270: F, t299: F, t47311: F, t706: F, t13945: F, t650: F, t43295: F, t43298: F, t43300: F, t43302: F, t43304: F, t47749: F, t47752: F) -> (F,) {
    let t47755 = t2508 * t2717 * t3722;
    let t47758 = t2508 * t954 * t12305;
    let t47764 = 0.76905262301422242837e-2 * t270 * t706 * t47311 * t169 * t299;
    let t47766 = 0.10254034973522965712e-1 * t650 * t13945;
    let t47767 = t43295 - 0.46143157380853345701e-1 * t43298 + t43300 - 0.53833683610995569986e-1 * t43302 - 0.53833683610995569986e-1 * t47749 + 0.10254034973522965712e-1 * t43304 + 0.76905262301422242837e-2 * t47752 + 0.76905262301422242837e-2 * t47755 + 0.76905262301422242837e-2 * t47758 + t47764 - t47766;
    (t47767,)
}
