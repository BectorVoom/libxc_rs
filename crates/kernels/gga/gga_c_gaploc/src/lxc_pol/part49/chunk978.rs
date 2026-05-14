//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 978/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk978<F: Float>(t2508: F, t2580: F, t47271: F, t12255: F, t1897: F, t7671: F, t12213: F, t7068: F, t13934: F, t731: F, t43055: F, t43087: F, t43090: F, t43094: F, t43096: F, t43099: F, t47640: F) -> (F,) {
    let t47644 = 0.15381052460284448567e-1 * t2508 * t2580 * t47271;
    let t47646 = t1897 * t12255 * t7671;
    let t47650 = t1897 * t2580 * t12213 * t7068;
    let t47652 = t731 * t13934;
    let t47656 = -t43055 + 0.10254034973522965712e-1 * t47640 + t47644 + 0.23071578690426672851e-1 * t47646 - 0.15381052460284448567e-1 * t47650 + 0.42725145723012357132e-3 * t47652 + 0.76905262301422242837e-2 * t43087 + 0.32043859292259267849e-3 * t43090 + t43094 - t43096 + t43099;
    (t47656,)
}
