//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 870/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk870<F: Float>(t1114: F, t6566: F, t3116: F, t6605: F, t343: F, t8890: F, t858: F, t2407: F, t2142: F, t3113: F, t1136: F, t6228: F) -> (F, F, F, F, F, F) {
    let t9119 = t1114 * t6566;
    let t9123 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3116 * t6605;
    let t9125 = t8890 * t343;
    let t9126 = t858 * t9125;
    let t9127 = t2407 * t9126;
    let t9142 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3113 * t2142;
    let t9144 = t6228 * t1136;
    (t9119, t9123, t9125, t9127, t9142, t9144)
}
