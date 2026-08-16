//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1288/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1288(t108879: f64, t2122: f64, t28150: f64, t8143: f64, t108978: f64, t108986: f64, t116: f64, t30715: f64, t2142: f64, t6628: f64, t3153: f64, t5219: f64, t7635: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t111639 = t2122 * t108879;
    let t111665 = t8143 * t28150;
    let t111670 = t2122 * t108978;
    let t111675 = t2122 * t108986;
    let t111696 = t30715 * t116;
    let t111814 = t2142 * t6628;
    let t111815 = t111814 * t3153;
    let t111832 = t5219 * t7635;
    (t111639, t111665, t111670, t111675, t111696, t111815, t111832)
}
