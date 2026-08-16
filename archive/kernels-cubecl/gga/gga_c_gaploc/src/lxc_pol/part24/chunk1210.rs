//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1210/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1210<F: Float>(t10682: F, t2042: F, t2508: F, t24722: F, t2541: F, t1897: F, t2580: F, t7068: F, t8469: F, t21455: F, t2958: F, t21460: F) -> (F, F, F, F, F) {
    let t32128 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2042 * t10682;
    let t32131 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t2541 * t24722;
    let t32135 = F::cast_from(0.30762104920568897134e-1_f64) * t1897 * t2580 * t8469 * t7068;
    let t32139 = F::cast_from(0.30762104920568897134e-1_f64) * t1897 * t2580 * t2958 * t21455;
    let t32143 = F::cast_from(0.15381052460284448567e-1_f64) * t1897 * t2580 * t2958 * t21460;
    (t32128, t32131, t32135, t32139, t32143)
}
