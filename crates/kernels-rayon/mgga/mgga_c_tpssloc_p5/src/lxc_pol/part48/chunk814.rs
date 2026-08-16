//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 814/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk814(t2113: f64, t2319: f64, t2363: f64, t23844: f64, t23846: f64, t23848: f64, t23850: f64, t23852: f64, t23854: f64, t24543: f64, t24932: f64, t671: f64, t7266: f64) -> (f64, f64) {
    let t24935 = t2113 * t2319;
    let t24939 = 2.0_f64 * t2363 * t7266 + 4.0_f64 * t24932 * t671 + t23844 + t23846 + t23848 + t23850 + t23852 + t23854 + t24543 + 2.0_f64 * t24935;
    (t24935, t24939)
}
