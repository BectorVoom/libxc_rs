//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1249/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1249(t11047: f64, t2197: f64, t2028: f64, t3038: f64, t7275: f64, t787: f64, t10012: f64, t10627: f64, t15482: f64, t22633: f64, t11053: f64, t7419: f64, t9805: f64) -> (f64, f64, f64, f64) {
    let t33136 = 0.23005755572352449806e2_f64 * t2197 * t11047;
    let t33145 = 0.79445533226334281486e-1_f64 * t787 * t7275 * t3038 * t2028;
    let t33148 = t10012 * t10627;
    let t33151 = 0.5680433474654925878e0_f64 * t22633 * t15482 * t33148;
    let t33153 = t9805 * t11053 * t7419;
    (t33136, t33145, t33151, t33153)
}
