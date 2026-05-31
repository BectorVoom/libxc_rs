//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1860/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1860<F: Float>(t4817: F, t7132: F, t25517: F, t25543: F, t25551: F, t25557: F, t25560: F, t25564: F, t27526: F, t27528: F, t27532: F, t27536: F, t4783: F, t4788: F, t4839: F) -> (F, F) {
    let t27539 = t7132 * t4817;
    let t27541 = F::cast_from(0.28582678745379824648e-3_f64) * t25517 * t4783 + F::cast_from(0.28582678745379824648e-3_f64) * t25517 * t4788 + t25543 / F::cast_from(864.0_f64) + F::cast_from(0.19055119163586549765e-3_f64) * t25551 - F::cast_from(0.15244095330869239812e-2_f64) * t25557 - t27526 * t27528 / F::cast_from(144.0_f64) + t27526 * t27532 / F::cast_from(216.0_f64) - t25560 + F::cast_from(0.28582678745379824648e-3_f64) * t25564 + F::cast_from(0.85748036236139473944e-3_f64) * t27536 * t4839 + F::cast_from(0.19055119163586549765e-3_f64) * t27539;
    (t27539, t27541)
}
