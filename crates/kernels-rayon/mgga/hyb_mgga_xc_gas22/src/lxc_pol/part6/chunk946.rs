//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 946/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk946(t809: f64, t8617: f64, t2234: f64, t2228: f64, t3356: f64, t1346: f64, t6564: f64, t2189: f64, t6562: f64, t1179: f64, t6536: f64, t2170: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8618 = t8617 * t809;
    let t8620 = 0.32163958997385070134e2_f64 * t2234 * t8618;
    let t8621 = t3356 * t2228;
    let t8623 = 0.16081979498692535067e2_f64 * t2234 * t8621;
    let t8624 = t1346 * t6564;
    let t8625 = t8624 * t2189;
    let t8627 = 0.51726012919273400301e3_f64 * t6562 * t8625;
    let t8632 = t6536 * t1179;
    let t8635 = t2170 * t3;
    (t8618, t8620, t8621, t8623, t8625, t8627, t8632, t8635)
}
