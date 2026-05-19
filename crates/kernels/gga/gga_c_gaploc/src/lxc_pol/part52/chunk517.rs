//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 517/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk517<F: Float>(t813: F, t9981: F, t3280: F, t549: F, t2033: F, t325: F, t40: F) -> (F, F, F) {
    let t9982 = t813 * t9981;
    let t10004 = t549 * t3280;
    let t10006 = F::cast_from(0.59584149919750711116e-1_f64) * t2033 * t10004;
    let t10007 = t40 * t325;
    (t9982, t10006, t10007)
}
