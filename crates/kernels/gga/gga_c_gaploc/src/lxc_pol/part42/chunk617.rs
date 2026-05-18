//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 617/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk617<F: Float>(t11627: F, t701: F, t2580: F, t2925: F, t2958: F, t10692: F, t10695: F, t10699: F, t10702: F, t10705: F, t11605: F, t11610: F, t11614: F, t11624: F, t1897: F, t2508: F) -> (F, F, F) {
    let t11628 = t11627 * t701;
    let t11629 = t2580 * t11628;
    let t11632 = t2958 * t2925;
    let t11633 = t2580 * t11632;
    let t11636 = -F::new(0.46143157380853345701e-1) * t2508 * t11605 + F::new(0.15381052460284448567e-1) * t2508 * t11610 + F::new(0.92286314761706691403e-1) * t2508 * t11614 - F::new(0.1281754371690370714e-2) * t10692 + F::new(0.2563508743380741428e-2) * t10695 - F::new(0.3845263115071112142e-2) * t10699 + F::new(0.1281754371690370714e-2) * t10702 + F::new(0.1281754371690370714e-2) * t10705 + F::new(0.76905262301422242837e-2) * t1897 * t11624 - F::new(0.15381052460284448567e-1) * t1897 * t11629 + F::new(0.30762104920568897134e-1) * t2508 * t11633;
    (t11628, t11632, t11636)
}
