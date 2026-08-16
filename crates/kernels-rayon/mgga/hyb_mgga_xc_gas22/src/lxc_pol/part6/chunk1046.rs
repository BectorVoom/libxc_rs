//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1046/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1046(t1181: f64, t2994: f64, t1230: f64, t2971: f64, t125: f64, t3804: f64, t545: f64, t3814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9827 = t1181 * t2994;
    let t9829 = t2971 * t1230;
    let t9833 = t125 * t3804;
    let t9834 = t9833 * t545;
    let t9838 = t125 * t3814;
    let t9839 = t9838 * t545;
    (t9827, t9829, t9833, t9834, t9838, t9839)
}
