//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1015/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1015<F: Float>(t3116: F, t6605: F, t6603: F, t343: F, t8890: F, t858: F, t2407: F, t6672: F, t2170: F, t875: F, t8961: F, t2168: F) -> (F, F, F, F, F, F) {
    let t9123 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3116 * t6605;
    let t9124 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t6603;
    let t9125 = t8890 * t343;
    let t9126 = t858 * t9125;
    let t9127 = t2407 * t9126;
    let t9129 = t6672 * t9127 / F::cast_from(24.0_f64);
    let t9131 = t2170 * t8961 * t875;
    let t9133 = t2168 * t9131 / F::cast_from(24.0_f64);
    (t9123, t9124, t9127, t9129, t9131, t9133)
}
