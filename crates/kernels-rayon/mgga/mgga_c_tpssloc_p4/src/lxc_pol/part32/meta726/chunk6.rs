//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2347/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2347(t26012: f64, t7974: f64, t1860: f64, t2109: f64, t22549: f64, t24514: f64, t26009: f64, t26024: f64, t26028: f64, t27303: f64, t27308: f64, t27365: f64, t27956: f64, t29481: f64, t6486: f64, t7255: f64, t7428: f64, t7975: f64, t7978: f64, t96045: f64, t96379: f64, t96458: f64) -> f64 {
    let t104787 = t7974 * t26012;
    let t104813 = -10.0_f64 * t96045 * t26009 - 10.0_f64 / 3.0_f64 * t22549 * t104787 - 10.0_f64 * t24514 * t96458 - t1860 * t7974 * t26024 / 3.0_f64 - t6486 * t29481 / 6.0_f64 - t1860 * t7255 * t27956 / 6.0_f64 - t1860 * t2109 * t96379 / 6.0_f64 - t26028 * t7975 / 3.0_f64 - t7428 * t27365 / 3.0_f64 - t7428 * t27303 / 3.0_f64 - t26028 * t7978 / 3.0_f64 - t7428 * t27308 / 3.0_f64;
    t104813
}
