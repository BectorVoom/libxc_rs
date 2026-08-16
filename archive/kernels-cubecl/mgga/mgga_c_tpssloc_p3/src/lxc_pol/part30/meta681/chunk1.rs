//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2139/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2139<F: Float>(t1992: F, t22897: F, t3792: F, t57607: F, t6378: F, t6990: F, t81039: F, t81047: F, t90845: F, t90860: F, t90865: F, t90867: F, t93538: F, t96962: F, t96967: F, t96972: F, t96976: F, t96979: F, t96986: F, t96989: F, t96993: F) -> F {
    let t96997 = t1992 * t22897 * t57607 * t3792;
    let t96999 = -F::cast_from(0.9869604401089358619e-1_f64) * t96962 - t90845 + t90860 + t90865 - t90867 + t93538 + F::cast_from(0.3289868133696452873e-1_f64) * t96967 + t6378 * t6990 + F::cast_from(0.16449340668482264365e-1_f64) * t96972 + F::cast_from(0.16449340668482264365e-1_f64) * t96976 - F::cast_from(0.16449340668482264365e-1_f64) * t96979 + F::cast_from(0.63969658155208805863e-1_f64) * t81039 - F::cast_from(0.26044789391763585244e-1_f64) * t81047 - F::cast_from(0.82246703342411321825e-2_f64) * t96986 + F::cast_from(0.41123351671205660912e-2_f64) * t96989 + F::cast_from(0.9869604401089358619e-1_f64) * t96993 + F::cast_from(0.16449340668482264365e-1_f64) * t96997;
    t96999
}
