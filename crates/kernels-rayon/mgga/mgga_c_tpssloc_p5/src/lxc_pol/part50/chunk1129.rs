//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1129/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1129(t225: f64, t25791: f64, t1921: f64, t7577: f64, t25820: f64, t23328: f64, t23394: f64, t4657: f64, t6703: f64, t25789: f64, t25822: f64, t28: f64, t40772: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t88145 = t25791 * t225;
    let t88162 = t7577 * t1921;
    let t88744 = t25820 * t225;
    let t88772 = t23328 * t23394;
    let t89598 = t6703 * t4657;
    let t89620 = t25789 * t225;
    let t89666 = t25822 * t225;
    let t89953 = t40772 * t28;
    (t88145, t88162, t88744, t88772, t89598, t89620, t89666, t89953)
}
