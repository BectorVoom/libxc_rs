//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 620/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk620<F: Float>(t8381: F, t8473: F, t8525: F, t8586: F, t103: F, t8460: F, t108: F, t1538: F, t1761: F, t1920: F, t438: F, t497: F, t7734: F, t7736: F, t8199: F, t8356: F, t8361: F, t8364: F, t8420: F, t8467: F, t8502: F, t88: F) -> (F, F, F) {
    let t8588 = t8381 + t8473 + t8525 + t8586;
    let t8590 = t8460 * t103;
    let t8598 = -t108 * t7734 - F::new(2.0) * t108 * t7736 - t108 * t8199 - F::new(3.0) * t1538 * t497 - F::new(3.0) * t1761 * t497 - F::new(3.0) * t1920 * t438 - t8588 * t88 - F::new(2.0) * t8356 - F::new(6.0) * t8361 - F::new(6.0) * t8364 - F::new(12.0) * t8420 + F::new(12.0) * t8467 + F::new(12.0) * t8502 + F::new(2.0) * t8590;
    (t8588, t8590, t8598)
}
