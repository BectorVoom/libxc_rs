//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1066/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1066(t2089: f64, t40: f64, t7291: f64, t15479: f64, t822: f64, t10007: f64, t15488: f64, t10012: f64, t1410: f64, t835: f64, t579: f64, t2683: f64, t5654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22628 = t822 * t15479;
    let t22629 = t10007 * t7291;
    let t22633 = t822 * t15488;
    let t22634 = t10012 * t7291;
    let t22672 = t1410 * t835;
    let t22693 = t579 * t2089;
    let t22706 = t579 * t835;
    let t22748 = t5654 * t2683;
    (t22623, t22624, t22628, t22629, t22633, t22634, t22672, t22693, t22706, t22748)
}
