//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1394/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1394(t7015: f64, t96334: f64, t7769: f64, t85416: f64, t24972: f64, t26550: f64, t116343: f64, t120809: f64, t120811: f64, t120812: f64, t120815: f64, t120818: f64, t120820: f64, t120823: f64, t5376: f64) -> f64 {
    let t123294 = t96334 * t7015;
    let t123296 = t85416 * t7769;
    let t123298 = t24972 * t26550;
    let t123304 = t120809 + t120811 + 27.0_f64 * t123294 + 27.0_f64 * t123296 + 27.0_f64 * t123298 + 27.0_f64 * t116343 * t5376 + 0.135e2_f64 * t120812 + 0.135e2_f64 * t120815 + t120818 + t120820 + t120823;
    t123304
}
