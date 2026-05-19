//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 943/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk943<F: Float>(t1897: F, t3266: F, t8942: F, t2508: F, t32658: F, t954: F, t40744: F, t40746: F, t43099: F, t43101: F, t43102: F, t43106: F, t43111: F, t43115: F, t43119: F, t43122: F, t43125: F, t43127: F, t43131: F, t43134: F, t43137: F, t43139: F) -> F {
    let t43143 = F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t3266 * t8942;
    let t43146 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t954 * t32658;
    let t43147 = F::cast_from(0.1281754371690370714e-2_f64) * t40744;
    let t43148 = F::cast_from(0.64087718584518535698e-3_f64) * t40746;
    let t43149 = t43099 + t43101 - F::cast_from(0.61524209841137794269e-1_f64) * t43102 - t43106 + t43111 + t43115 - t43119 + t43122 - t43125 + F::cast_from(0.64087718584518535698e-3_f64) * t43127 + t43131 - F::cast_from(0.30762104920568897134e-1_f64) * t43134 - F::cast_from(0.15381052460284448567e-1_f64) * t43137 + F::cast_from(0.85450291446024714264e-3_f64) * t43139 - t43143 + t43146 + t43147 - t43148;
    t43149
}
