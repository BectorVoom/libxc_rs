//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 860/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk860<F: Float>(t1897: F, t28957: F, t2936: F, t10782: F, t2580: F, t7068: F, t32112: F, t954: F, t13225: F, t731: F, t3266: F, t8942: F, t2508: F, t32658: F, t40744: F, t40746: F) -> (F, F, F, F, F, F, F, F) {
    let t43131 = 0.23071578690426672851e-1 * t1897 * t2936 * t28957;
    let t43134 = t1897 * t2580 * t10782 * t7068;
    let t43137 = t1897 * t954 * t32112;
    let t43139 = t731 * t13225;
    let t43143 = 0.76905262301422242837e-2 * t1897 * t3266 * t8942;
    let t43146 = 0.15381052460284448567e-1 * t2508 * t954 * t32658;
    let t43147 = 0.1281754371690370714e-2 * t40744;
    let t43148 = 0.64087718584518535698e-3 * t40746;
    (t43131, t43134, t43137, t43139, t43143, t43146, t43147, t43148)
}
