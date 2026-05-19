//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 609/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk609<F: Float>(t1590: F, t458: F, t1159: F, t164: F, t479: F, t695: F, t1: F, t1750: F, t726: F, t1755: F, t116: F, t717: F, t732: F) -> (F, F, F, F, F, F) {
    let t4275 = F::cast_from(0.09451622166942335_f64) * t458 * t1590;
    let t4276 = t1159 * t164;
    let t4279 = F::cast_from(0.1890324433388467_f64) * t695 * t479;
    let t4291 = t726 * t1750 * t1;
    let t4292 = t4291 * t1755;
    let t4293 = F::cast_from(1.898172889849454_f64) * t4292;
    let t4295 = t732 * t717 * t116;
    (t4275, t4276, t4279, t4291, t4293, t4295)
}
