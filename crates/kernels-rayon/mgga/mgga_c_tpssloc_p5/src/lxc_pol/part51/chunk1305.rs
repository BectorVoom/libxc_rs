//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1305/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1305(t4234: f64, t6605: f64, t6612: f64, t25119: f64, t4255: f64, t6619: f64, t23046: f64, t25093: f64, t23097: f64, t25097: f64, t112792: f64, t4184: f64) -> (f64, f64, f64, f64, f64) {
    let t118549 = t6605 * t6612 * t4234;
    let t118552 = t25119 * t6619 * t4255;
    let t118556 = t6605 * t23046 * t25093;
    let t118559 = t23097 * t6612 * t25097;
    let t118562 = t112792 * t4184;
    (t118549, t118552, t118556, t118559, t118562)
}
