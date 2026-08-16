//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1351/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1351(t24702: f64, t8614: f64, t8621: f64, t8906: f64, t24712: f64, t8625: f64, t2228: f64, t2234: f64, t4114: f64, t2189: f64, t4143: f64, t6562: f64) -> (f64, f64, f64, f64, f64) {
    let t29438 = 0.19298375398431042081e3_f64 * t24702 * t8614;
    let t29440 = 0.32163958997385070134e2_f64 * t8906 * t8621;
    let t29442 = 0.1034520258385468006e4_f64 * t24712 * t8625;
    let t29445 = 6.0_f64 * t2234 * t4114 * t2228;
    let t29448 = 0.57895126195293126241e3_f64 * t6562 * t4143 * t2189;
    (t29438, t29440, t29442, t29445, t29448)
}
