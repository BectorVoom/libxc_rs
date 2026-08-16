//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1064/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1064(t15488: f64, t822: f64, t10012: f64, t7291: f64, t1410: f64, t835: f64, t2089: f64, t579: f64, t2683: f64, t5654: f64, t1890: f64, t21783: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22633 = t822 * t15488;
    let t22634 = t10012 * t7291;
    let t22672 = t1410 * t835;
    let t22693 = t579 * t2089;
    let t22706 = t579 * t835;
    let t22748 = t5654 * t2683;
    let t22775 = t1890 * t21783;
    (t22633, t22634, t22672, t22693, t22706, t22748, t22775)
}
