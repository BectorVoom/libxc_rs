//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1133/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1133(t254: f64, t382: f64, t10164: f64, t1955: f64, t225: f64, t7569: f64, t1921: f64, t25749: f64, t7561: f64, t968: f64, t1920: f64, t1625: f64, t6688: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    let t25778 = t7569 * t225;
    let t25784 = t1921 * t25749;
    let t25806 = t968 * t7561;
    let t25807 = t1920 * t25806;
    let t25810 = t6688 * t1625;
    (t25757, t25758, t25778, t25784, t25807, t25810)
}
