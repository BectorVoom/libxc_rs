//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1042/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1042<F: Float>(t2182: F, t3178: F, t5: F, t9079: F, t1112: F, t745: F, t343: F, t810: F, t8961: F, t2074: F, t2118: F, t8913: F, t2352: F, t8589: F, t2100: F, t1105: F, t2079: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27363 = t3178 * t2182;
    let t27618 = t5 * t9079;
    let t27691 = t1112 * t745;
    let t27823 = t27618 * t343;
    let t28024 = t8961 * t810;
    let t28029 = t3178 * t2074;
    let t28139 = t2118 * t8913;
    let t28457 = t8589 * t2352;
    let t28647 = t2118 * t2100;
    let t28667 = t1105 * t2079;
    (t27363, t27691, t27823, t28024, t28029, t28139, t28457, t28647, t28667)
}
