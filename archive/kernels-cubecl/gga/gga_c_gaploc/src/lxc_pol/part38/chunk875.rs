//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 875/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk875<F: Float>(t2508: F, t37060: F, t948: F, t13535: F, t7137: F, t13525: F, t835: F, t723: F, t13507: F, t7129: F, t2717: F, t3603: F) -> (F, F, F, F, F, F, F) {
    let t44936 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t37060 * t948;
    let t44938 = F::cast_from(0.10254034973522965712e-1_f64) * t7137 * t13535;
    let t44939 = t835 * t13525;
    let t44940 = t44939 * t723;
    let t44956 = F::cast_from(0.61524209841137794268e-1_f64) * t7137 * t13507;
    let t44960 = F::cast_from(0.76905262301422242837e-2_f64) * t7129 * t13535;
    let t44963 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2717 * t3603;
    (t44936, t44938, t44939, t44940, t44956, t44960, t44963)
}
