//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1142/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1142<F: Float>(t19933: F, t8054: F, t2592: F, t8854: F, t2969: F, t7817: F, t7329: F, t8862: F, t1960: F, t977: F, t1382: F, t2497: F, t2902: F, t16710: F, t1961: F, t3459: F) -> (F, F, F, F, F, F, F) {
    let t33979 = 6.0 * t19933 * t8054;
    let t33980 = t2592 * t8854;
    let t33982 = t2969 * t7817;
    let t33988 = 4.0 * t8862 * t7329;
    let t33991 = 2.0 * t1960 * t8854 * t977;
    let t33997 = 4.0 * t1382 * t2902 * t2497;
    let t34003 = 24.0 * t16710 * t3459 * t1961;
    (t33979, t33980, t33982, t33988, t33991, t33997, t34003)
}
