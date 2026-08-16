//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1751/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1751(t2109: f64, t22550: f64, t7245: f64, t9231: f64, t33: f64, t7254: f64, t2240: f64, t1235: f64, t7299: f64, t2127: f64, t23383: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24517 = t2109 * t22550;
    let t24520 = t9231 * t7245;
    let t24525 = t33 * t7254;
    let t24526 = t2240 * t24525;
    let t24567 = t7299 * t1235;
    let t24574 = t2127 * t23383;
    (t24517, t24520, t24525, t24526, t24567, t24574)
}
