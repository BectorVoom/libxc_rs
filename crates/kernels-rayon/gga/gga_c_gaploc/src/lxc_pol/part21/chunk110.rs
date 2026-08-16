//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 110/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk110(t1: f64, t44: f64, t343: f64, t55: f64, t78: f64, t46: f64, t51: f64, t345: f64, t347: f64, t351: f64, t353: f64, t54: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t360 = t44 * t1;
    let t362 = t343 * t78 * t55;
    let t364 = 0.18311555036753159941e-3_f64 * t360 * t362;
    let t365 = t44 * t46;
    let t366 = t51 * t51;
    let t367 = 1.0_f64 / t366;
    let t372 = -0.86308333333333333334e0_f64 * t345 - 0.301925e0_f64 * t347 - 0.5501625e-1_f64 * t351 - 0.82785e-1_f64 * t353;
    let t374 = 1.0_f64 / t54;
    (t360, t362, t364, t365, t366, t367, t372, t374)
}
