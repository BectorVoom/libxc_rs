//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1207/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1207(t1230: f64, t1861: f64, t10: f64, t16: f64, t6610: f64, t2951: f64, t639: f64, t7834: f64, t1806: f64, t92: f64, t2970: f64, t7837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23023 = t1230 * t1861;
    let t23029 = t6610 * t10 * t16;
    let t23030 = t23029 * t2951;
    let t23043 = t7834 * t639;
    let t23048 = t6610 * t1806 * t92;
    let t23050 = t2970 * t23048 * t7837;
    (t23023, t23029, t23030, t23043, t23048, t23050)
}
