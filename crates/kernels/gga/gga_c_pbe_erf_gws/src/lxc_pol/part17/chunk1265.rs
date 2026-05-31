//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1265/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1265<F: Float>(t53583: F, t22509: F, t4166: F, t1176: F, t21518: F, t367: F, t3974: F, t3990: F, t8939: F, t14602: F, t51666: F, t3959: F, t9704: F) -> (F, F, F, F, F) {
    let t53584 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t53583;
    let t53585 = t22509 * t4166;
    let t53592 = t1176 * t367 * t21518;
    let t53595 = t53592 * t3990 * t3974 * t8939;
    let t53597 = t51666 * t14602;
    let t53598 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53597;
    let t53599 = t3959 * t9704;
    (t53584, t53585, t53595, t53598, t53599)
}
