//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 848/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk848<F: Float>(t13918: F, t7129: F, t2508: F, t2717: F, t3722: F, t12305: F, t954: F, t169: F, t270: F, t299: F, t47311: F, t706: F, t13945: F, t650: F, t13934: F, t2549: F) -> (F, F, F, F, F, F) {
    let t47752 = t7129 * t13918;
    let t47755 = t2508 * t2717 * t3722;
    let t47758 = t2508 * t954 * t12305;
    let t47764 = 0.76905262301422242837e-2 * t270 * t706 * t47311 * t169 * t299;
    let t47766 = 0.10254034973522965712e-1 * t650 * t13945;
    let t47768 = t2549 * t13934;
    (t47752, t47755, t47758, t47764, t47766, t47768)
}
