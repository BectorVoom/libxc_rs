//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 738/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk738<F: Float>(t13489: F, t731: F, t13495: F, t7137: F, t13486: F, t7129: F, t2508: F, t37060: F, t948: F, t13535: F, t13507: F, t2717: F, t3603: F, t13556: F, t35709: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44928 = t731 * t13489;
    let t44931 = 0.10254034973522965712e-1 * t7137 * t13495;
    let t44933 = 0.23071578690426672851e-1 * t7129 * t13486;
    let t44936 = 0.23071578690426672851e-1 * t2508 * t37060 * t948;
    let t44938 = 0.10254034973522965712e-1 * t7137 * t13535;
    let t44956 = 0.61524209841137794268e-1 * t7137 * t13507;
    let t44960 = 0.76905262301422242837e-2 * t7129 * t13535;
    let t44963 = 0.76905262301422242837e-2 * t2508 * t2717 * t3603;
    let t44972 = 0.15381052460284448567e-1 * t7129 * t13556;
    let t44973 = t35709 * t935;
    (t44928, t44931, t44933, t44936, t44938, t44956, t44960, t44963, t44972, t44973)
}
