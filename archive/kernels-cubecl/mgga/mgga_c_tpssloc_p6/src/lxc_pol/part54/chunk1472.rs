//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1472/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1472<F: Float>(t116135: F, t121240: F, t121253: F, t121254: F, t122084: F, t122088: F, t122094: F, t122583: F, t122587: F, t122589: F, t122590: F, t122593: F, t2165: F, t26872: F, t26974: F, t27170: F, t652: F) -> F {
    let t124977 = -F::cast_from(2.0_f64) * t2165 * t27170 * t652 - F::cast_from(3.0_f64) * t116135 * t26872 - F::cast_from(3.0_f64) * t116135 * t26974 - t121240 - t121253 - t121254 - t122084 + t122088 + t122094 + t122583 + t122587 - t122589 - t122590 - t122593;
    t124977
}
