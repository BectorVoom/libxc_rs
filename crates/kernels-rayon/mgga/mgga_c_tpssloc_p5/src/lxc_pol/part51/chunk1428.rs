//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1428/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1428(t115296: f64, t1799: f64, t22633: f64, t22635: f64, t2086: f64, t254: f64, t33297: f64, t6883: f64, t115545: f64, t26338: f64, t120240: f64, t31558: f64) -> (f64, f64, f64, f64, f64) {
    let t122204 = t22633 * t22635 * t115296 * t1799;
    let t122206 = t2086 * t254;
    let t122210 = t6883 * t33297;
    let t122213 = t22633 * t115545 * t26338;
    let t122218 = t22633 * t22635 * t31558 * t120240;
    (t122204, t122206, t122210, t122213, t122218)
}
