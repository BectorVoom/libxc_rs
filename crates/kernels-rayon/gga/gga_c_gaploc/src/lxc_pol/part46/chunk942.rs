//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 942/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk942(t1897: f64, t3266: f64, t8942: f64, t2508: f64, t32658: f64, t954: f64, t40744: f64, t40746: f64, t43099: f64, t43101: f64, t43102: f64, t43106: f64, t43111: f64, t43115: f64, t43119: f64, t43122: f64, t43125: f64, t43127: f64, t43131: f64, t43134: f64, t43137: f64, t43139: f64) -> f64 {
    let t43143 = 0.76905262301422242837e-2_f64 * t1897 * t3266 * t8942;
    let t43146 = 0.15381052460284448567e-1_f64 * t2508 * t954 * t32658;
    let t43147 = 0.1281754371690370714e-2_f64 * t40744;
    let t43148 = 0.64087718584518535698e-3_f64 * t40746;
    let t43149 = t43099 + t43101 - 0.61524209841137794269e-1_f64 * t43102 - t43106 + t43111 + t43115 - t43119 + t43122 - t43125 + 0.64087718584518535698e-3_f64 * t43127 + t43131 - 0.30762104920568897134e-1_f64 * t43134 - 0.15381052460284448567e-1_f64 * t43137 + 0.85450291446024714264e-3_f64 * t43139 - t43143 + t43146 + t43147 - t43148;
    t43149
}
