//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 984/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk984<F: Float>(t13879: F, t1897: F, t702: F, t13941: F, t2508: F, t779: F, t13945: F, t681: F, t13942: F, t650: F, t270: F, t47420: F, t738: F) -> (F, F, F, F, F) {
    let t47616 = F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t13879 * t702;
    let t47619 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t779 * t13941;
    let t47629 = F::cast_from(0.76905262301422242837e-2_f64) * t681 * t13945;
    let t47631 = F::cast_from(0.10254034973522965712e-1_f64) * t650 * t13942;
    let t47634 = F::cast_from(0.76905262301422242837e-2_f64) * t270 * t738 * t47420;
    (t47616, t47619, t47629, t47631, t47634)
}
