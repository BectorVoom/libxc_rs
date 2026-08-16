//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1063/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1063(t1984: f64, t22537: f64, t823: f64, t9419: f64, t15478: f64, t5638: f64, t822: f64, t2089: f64, t40: f64, t7291: f64, t15479: f64, t10007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22538 = t1984 * t22537;
    let t22542 = t823 * t9419;
    let t22543 = t1984 * t22542;
    let t22622 = t822 * t5638 * t15478;
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22628 = t822 * t15479;
    let t22629 = t10007 * t7291;
    (t22538, t22542, t22543, t22622, t22623, t22624, t22628, t22629)
}
