//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1419/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1419(t1863: f64, t83728: f64, t1860: f64, t1865: f64, t22489: f64, t22490: f64, t22493: f64, t22549: f64, t22551: f64, t6486: f64, t6505: f64, t6506: f64, t6510: f64, t83699: f64, t83706: f64, t83710: f64, t83717: f64, t83719: f64, t83722: f64, t83725: f64) -> f64 {
    let t83729 = t1863 * t83728;
    let t83732 = t83699 * t1865 - t6486 * t22490 / 2.0_f64 - t1860 * t6505 * t22489 / 2.0_f64 - t1860 * t1863 * t83706 / 6.0_f64 - t83710 * t1865 / 6.0_f64 - t22493 * t6506 / 2.0_f64 - t22493 * t6510 / 2.0_f64 + 30.0_f64 * t83717 * t83719 - 10.0_f64 * t83722 * t22551 - 10.0_f64 * t22549 * t83725 - 10.0_f64 * t22549 * t83729;
    t83732
}
