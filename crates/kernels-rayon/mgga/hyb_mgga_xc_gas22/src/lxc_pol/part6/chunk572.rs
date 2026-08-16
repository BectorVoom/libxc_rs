//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 572/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk572(t10: f64, t1067: f64, t1107: f64, t1030: f64, t567: f64, t1048: f64, t222: f64, t16: f64, t1884: f64, t492: f64) -> (f64, f64, f64, f64, f64) {
    let t2699 = t1067 * t10;
    let t2700 = t2699 * t1107;
    let t2702 = t567 * t1030;
    let t2705 = 0.35616666666666666666e-1_f64 * t222 * t2702 * t1048;
    let t2707 = t16 * t1884 * t492;
    (t2699, t2700, t2702, t2705, t2707)
}
