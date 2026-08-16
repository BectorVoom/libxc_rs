//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1063/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1063(t1892: f64, t8477: f64, t1903: f64, t8578: f64, t32250: f64, t1882: f64, t543: f64, t32255: f64, t2022: f64, t7910: f64, t8707: f64, t8590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33943 = t8477 * t1892;
    let t33946 = t8578 * t1903;
    let t33947 = t32250 * t33946;
    let t33951 = t8578 * t1882 * t543;
    let t33952 = t32255 * t33951;
    let t33955 = t2022 * t7910;
    let t33956 = t8707 * t33955;
    let t33959 = t33943 * t8590;
    (t33943, t33946, t33947, t33951, t33952, t33955, t33956, t33959)
}
