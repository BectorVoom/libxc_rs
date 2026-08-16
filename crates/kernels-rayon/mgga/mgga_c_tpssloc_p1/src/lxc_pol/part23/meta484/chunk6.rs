//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1478/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1478(t491: f64, t79018: f64, t11881: f64, t11883: f64, t11888: f64, t15027: f64, t1729: f64, t22349: f64, t22358: f64, t22368: f64, t22369: f64, t22375: f64, t22387: f64, t3508: f64, t3610: f64, t44753: f64, t44754: f64, t44785: f64, t44786: f64, t470: f64, t493: f64, t5064: f64, t53592: f64, t53613: f64, t6224: f64, t6256: f64, t6260: f64, t6739: f64, t79391: f64, t79410: f64) -> (f64, f64) {
    let t79473 = t491 * t79018;
    let t79524 = -36.0_f64 * t11888 * t3508 * t6224 * t6260 * t6739 + 24.0_f64 * t11881 * t11883 * t79410 + 24.0_f64 * t22368 * t3610 * t6256 + 14.0_f64 * t44753 * t44754 * t79473 - t44785 * t44786 * t79473 + t470 * t493 * t79391 + 24.0_f64 * t15027 * t22369 + 4.0_f64 * t1729 * t22375 + 4.0_f64 * t22349 * t53592 + 24.0_f64 * t22358 * t53613 + 4.0_f64 * t22387 * t5064;
    (t79473, t79524)
}
