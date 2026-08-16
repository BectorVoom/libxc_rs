//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 651/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk651(t11627: f64, t701: f64, t2580: f64, t2925: f64, t2958: f64, t10692: f64, t10695: f64, t10699: f64, t10702: f64, t10705: f64, t11605: f64, t11610: f64, t11614: f64, t11624: f64, t1897: f64, t2508: f64) -> (f64, f64, f64) {
    let t11628 = t11627 * t701;
    let t11629 = t2580 * t11628;
    let t11632 = t2958 * t2925;
    let t11633 = t2580 * t11632;
    let t11636 = -0.46143157380853345701e-1_f64 * t2508 * t11605 + 0.15381052460284448567e-1_f64 * t2508 * t11610 + 0.92286314761706691403e-1_f64 * t2508 * t11614 - 0.1281754371690370714e-2_f64 * t10692 + 0.2563508743380741428e-2_f64 * t10695 - 0.3845263115071112142e-2_f64 * t10699 + 0.1281754371690370714e-2_f64 * t10702 + 0.1281754371690370714e-2_f64 * t10705 + 0.76905262301422242837e-2_f64 * t1897 * t11624 - 0.15381052460284448567e-1_f64 * t1897 * t11629 + 0.30762104920568897134e-1_f64 * t2508 * t11633;
    (t11628, t11632, t11636)
}
