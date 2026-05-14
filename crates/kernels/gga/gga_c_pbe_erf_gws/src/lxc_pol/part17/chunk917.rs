//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 917/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk917<F: Float>(t858: F, t9125: F, t2407: F, t6672: F, t2170: F, t875: F, t8961: F, t2168: F, t2190: F, t3178: F, t6606: F, t6597: F, t9110: F, t9113: F, t9114: F, t9118: F, t9121: F, t9123: F, t9124: F) -> (F, F, F, F, F, F, F, F) {
    let t9126 = t858 * t9125;
    let t9127 = t2407 * t9126;
    let t9129 = t6672 * t9127 / 24.0;
    let t9131 = t2170 * t8961 * t875;
    let t9133 = t2168 * t9131 / 24.0;
    let t9135 = t2170 * t3178 * t2190;
    let t9137 = t2168 * t9135 / 48.0;
    let t9138 = 7.0 / 288.0 * t6606;
    let t9139 = -t9110 - t9113 - t9114 - t9118 + t9121 + t9123 - t6597 - t9124 + t9129 + t9133 + t9137 + t9138;
    (t9127, t9129, t9131, t9133, t9135, t9137, t9138, t9139)
}
