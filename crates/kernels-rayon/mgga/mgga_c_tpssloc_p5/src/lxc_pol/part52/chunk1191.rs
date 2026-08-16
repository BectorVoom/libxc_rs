//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1191/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1191(t6880: f64, t8690: f64, t2165: f64, t6534: f64, t652: f64, t31247: f64, t31249: f64, t31250: f64, t31880: f64, t31898: f64, t31900: f64, t31902: f64, t31904: f64, t31906: f64, t31909: f64, t31913: f64, t6539: f64, t672: f64, t7266: f64) -> (f64, f64) {
    let t31916 = t8690 * t6880;
    let t31918 = t2165 * t6534;
    let t31919 = t652 * t31918;
    let t31921 = -2.0_f64 * t31880 * t672 - 2.0_f64 * t31913 * t652 - 2.0_f64 * t6539 * t7266 + t31247 - t31249 + t31250 - 2.0_f64 * t31898 - 2.0_f64 * t31900 - 2.0_f64 * t31902 - 2.0_f64 * t31904 - 2.0_f64 * t31906 - 2.0_f64 * t31909 + 3.0_f64 * t31916 - 2.0_f64 * t31919;
    (t31918, t31921)
}
