//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1119/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1119(t1006: f64, t11003: f64, t997: f64, t1007: f64, t4344: f64, t3482: f64, t9258: f64, t3518: f64, t9104: f64, t4244: f64, t967: f64, t2521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11005 = t997 * t11003 * t1006;
    let t11008 = t4344 * t1007;
    let t11016 = 4.0_f64 * t9258 * t3482;
    let t11018 = 0.32163958997385070134e2_f64 * t9104 * t3518;
    let t11019 = t4244 * t967;
    let t11021 = 6.0_f64 * t2521 * t11019;
    (t11005, t11008, t11016, t11018, t11019, t11021)
}
