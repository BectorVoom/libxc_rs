//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 807/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk807(t12461: f64, t3698: f64, t2019: f64, t1983: f64, t113: f64, t1976: f64, t22594: f64, t22599: f64, t22600: f64, t22605: f64, t22608: f64, t22610: f64, t22612: f64, t22614: f64, t22616: f64, t22618: f64, t22619: f64, t22950: f64, t2312: f64, t2364: f64, t23829: f64, t23833: f64, t23835: f64, t23837: f64, t23855: f64, t510: f64, t574: f64, t6517: f64, t652: f64) -> (f64, f64, f64) {
    let t23857 = t12461 * t3698;
    let t23858 = t2019 * t23857;
    let t23860 = 2.0_f64 * t1983 * t23858;
    let t23861 = -t113 * t23829 - t1976 * t2312 - 2.0_f64 * t22600 * t510 - 4.0_f64 * t22619 * t652 - 2.0_f64 * t2364 * t6517 + t23855 * t574 + t22594 + t22599 + t22605 + t22608 - t22610 - t22612 - t22614 - t22616 - t22618 + t22950 - t23833 - t23835 + t23837 + t23860;
    (t23857, t23858, t23861)
}
