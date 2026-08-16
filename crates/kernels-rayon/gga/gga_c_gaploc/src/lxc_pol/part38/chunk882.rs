//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 882/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk882(t13495: f64, t7129: f64, t2508: f64, t2717: f64, t3616: f64, t11588: f64, t954: f64, t10667: f64, t2958: f64, t1035: f64, t1897: f64, t2580: f64, t3009: f64, t32112: f64, t32214: f64, t32260: f64, t43196: f64, t43224: f64, t45029: f64, t45031: f64, t45034: f64, t45037: f64, t45044: f64, t45048: f64, t45052: f64, t45054: f64, t45057: f64, t7226: f64) -> (f64, f64) {
    let t45059 = 0.76905262301422242837e-2_f64 * t7129 * t13495;
    let t45062 = 0.76905262301422242837e-2_f64 * t2508 * t2717 * t3616;
    let t45065 = 0.76905262301422242837e-2_f64 * t2508 * t954 * t11588;
    let t45066 = t2958 * t10667;
    let t45070 = -0.15381052460284448567e-1_f64 * t1897 * t1035 * t32112 + 0.1281754371690370714e-2_f64 * t43196 - 0.30762104920568897134e-1_f64 * t1897 * t2580 * t2958 * t32260 - t45029 + t45031 - t45034 + t45037 - 0.92286314761706691402e-1_f64 * t2508 * t7226 * t3009 * t32214 + t45044 + t45048 + 0.1281754371690370714e-2_f64 * t43224 + t45052 - 0.64087718584518535696e-3_f64 * t45054 - t45057 + t45059 + t45062 + t45065 + 0.30762104920568897134e-1_f64 * t2508 * t2580 * t45066;
    (t45066, t45070)
}
