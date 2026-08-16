//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1533/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1533(t1668: f64, t905: f64, t11774: f64, t53391: f64, t6267: f64, t19968: f64, t4817: f64, t20054: f64, t4834: f64, t19882: f64, t1062: f64, t23960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t79450 = t1668 * t905;
    let t79474 = t11774 * t53391 * t6267;
    let t79546 = t19968 * t4817;
    let t79548 = t4834 * t20054;
    let t79553 = t4834 * t19882;
    let t79559 = t23960 * t1062;
    (t79450, t79474, t79546, t79548, t79553, t79559)
}
