//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 882/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk882<F: Float>(t13495: F, t7129: F, t2508: F, t2717: F, t3616: F, t11588: F, t954: F, t10667: F, t2958: F, t1035: F, t1897: F, t2580: F, t3009: F, t32112: F, t32214: F, t32260: F, t43196: F, t43224: F, t45029: F, t45031: F, t45034: F, t45037: F, t45044: F, t45048: F, t45052: F, t45054: F, t45057: F, t7226: F) -> (F, F) {
    let t45059 = F::cast_from(0.76905262301422242837e-2_f64) * t7129 * t13495;
    let t45062 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2717 * t3616;
    let t45065 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t954 * t11588;
    let t45066 = t2958 * t10667;
    let t45070 = -F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t1035 * t32112 + F::cast_from(0.1281754371690370714e-2_f64) * t43196 - F::cast_from(0.30762104920568897134e-1_f64) * t1897 * t2580 * t2958 * t32260 - t45029 + t45031 - t45034 + t45037 - F::cast_from(0.92286314761706691402e-1_f64) * t2508 * t7226 * t3009 * t32214 + t45044 + t45048 + F::cast_from(0.1281754371690370714e-2_f64) * t43224 + t45052 - F::cast_from(0.64087718584518535696e-3_f64) * t45054 - t45057 + t45059 + t45062 + t45065 + F::cast_from(0.30762104920568897134e-1_f64) * t2508 * t2580 * t45066;
    (t45066, t45070)
}
