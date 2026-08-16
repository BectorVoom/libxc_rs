//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2147/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2147(t1992: f64, t22897: f64, t3792: f64, t57607: f64, t6378: f64, t6990: f64, t81039: f64, t81047: f64, t90845: f64, t90860: f64, t90865: f64, t90867: f64, t93538: f64, t96962: f64, t96967: f64, t96972: f64, t96976: f64, t96979: f64, t96986: f64, t96989: f64, t96993: f64) -> f64 {
    let t96997 = t1992 * t22897 * t57607 * t3792;
    let t96999 = -0.9869604401089358619e-1_f64 * t96962 - t90845 + t90860 + t90865 - t90867 + t93538 + 0.3289868133696452873e-1_f64 * t96967 + t6378 * t6990 + 0.16449340668482264365e-1_f64 * t96972 + 0.16449340668482264365e-1_f64 * t96976 - 0.16449340668482264365e-1_f64 * t96979 + 0.63969658155208805863e-1_f64 * t81039 - 0.26044789391763585244e-1_f64 * t81047 - 0.82246703342411321825e-2_f64 * t96986 + 0.41123351671205660912e-2_f64 * t96989 + 0.9869604401089358619e-1_f64 * t96993 + 0.16449340668482264365e-1_f64 * t96997;
    t96999
}
