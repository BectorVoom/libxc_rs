//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1457/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1457(t120874: f64, t123111: f64, t1774: f64, t2036: f64, t2096: f64, t2314: f64, t23938: f64, t26977: f64, t27147: f64, t27188: f64, t27219: f64, t27858: f64, t27879: f64, t32349: f64, t32365: f64, t34170: f64, t5107: f64, t7042: f64, t7266: f64, t7271: f64, t7458: f64, t7989: f64, t8829: f64) -> f64 {
    let t124708 = t123111 * t2096 - t1774 * t32349 - t2036 * t27858 - 2.0_f64 * t2314 * t34170 - 2.0_f64 * t23938 * t7989 - 2.0_f64 * t26977 * t7989 - 2.0_f64 * t27147 * t7266 - 2.0_f64 * t27188 * t7271 - 2.0_f64 * t27219 * t7266 - 2.0_f64 * t27879 * t7042 - 2.0_f64 * t32365 * t7458 - t5107 * t8829 + t120874;
    t124708
}
