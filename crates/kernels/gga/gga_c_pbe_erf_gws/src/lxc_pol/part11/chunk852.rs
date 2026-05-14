//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 852/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk852<F: Float>(t4422: F, t885: F, t6158: F, t6670: F, t6587: F, t899: F, t900: F, t6045: F, t855: F, t863: F, t1: F, t6382: F, t253: F, t2118: F, t8986: F, t6201: F, t916: F) -> (F, F, F, F, F, F, F) {
    let t21491 = t4422 * t885;
    let t21497 = t6158 * t6670;
    let t21507 = t899 * t900 * t6587;
    let t21511 = t863 * t855 * t6045;
    let t21518 = t6382 * t1;
    let t21519 = t21518 * t253;
    let t21529 = t2118 * t8986;
    let t21535 = t6201 * t916;
    (t21491, t21497, t21507, t21511, t21519, t21529, t21535)
}
