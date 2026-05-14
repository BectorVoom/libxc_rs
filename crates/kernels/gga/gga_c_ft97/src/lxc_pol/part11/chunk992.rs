//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 992/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk992<F: Float>(t10568: F, t1775: F, t2739: F, t505: F, t11176: F, t303: F, t10607: F, t10362: F, t289: F, t287: F, t2726: F, t2735: F, t41451: F, t41456: F, t41461: F, t41466: F, t41471: F, t41475: F, t41480: F, t41484: F, t41488: F, t41492: F, t41495: F) -> (F, F, F, F, F, F, F, F) {
    let t43563 = t1775 * t10568;
    let t43568 = t505 * t2739;
    let t43574 = 280.0 / 81.0 * t11176 * t303;
    let t43578 = t1775 * t10607;
    let t43585 = 1.0 / t10362 / t289;
    let t43586 = t287 * t43585;
    let t43587 = t2726 * t2726;
    let t43595 = t2735 * t2735;
    let t43626 = -0.62232801019753086422e0 * t41451 + 0.31116400509876543211e0 * t41456 + 0.80013601311111111114e0 * t41461 - 0.80013601311111111114e0 * t41466 + 0.66678001092592592595e-1 * t41471 + 0.8890400145679012346e-1 * t41475 - 0.40006800655555555556e0 * t41480 + 0.60010200983333333334e0 * t41484 - 0.10001700163888888889e0 * t41488 - 0.13335600218518518519e0 * t41492 + 0.44452000728395061732e-1 * t41495;
    (t43563, t43568, t43574, t43578, t43586, t43587, t43595, t43626)
}
