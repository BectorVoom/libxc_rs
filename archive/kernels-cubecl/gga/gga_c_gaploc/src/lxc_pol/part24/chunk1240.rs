//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1240/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1240<F: Float>(t10763: F, t7129: F, t2508: F, t2717: F, t2927: F, t8979: F, t954: F, t21636: F, t3448: F, t21571: F, t10714: F, t10718: F) -> (F, F, F, F, F, F, F) {
    let t32529 = F::cast_from(0.46143157380853345702e-1_f64) * t7129 * t10763;
    let t32532 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2717 * t2927;
    let t32535 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t954 * t8979;
    let t32539 = F::cast_from(0.6836023315681977141e-2_f64) * t21636 * t3448;
    let t32541 = F::cast_from(0.15381052460284448567e-1_f64) * t21571 * t3448;
    let t32543 = F::cast_from(0.30762104920568897134e-1_f64) * t7129 * t10714;
    let t32545 = F::cast_from(0.30762104920568897134e-1_f64) * t7129 * t10718;
    (t32529, t32532, t32535, t32539, t32541, t32543, t32545)
}
