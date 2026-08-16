//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 989/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk989(t2132: f64, t2138: f64, t322: f64, t8301: f64, t2230: f64, t29985: f64, t7987: f64, t8104: f64, t2131: f64, t3644: f64, t633: f64, t2217: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t33262 = t2138 * t2132 * t8301 * t322;
    let t33264 = t29985 * t2230;
    let t33266 = t7987 * t8104;
    let t33271 = 0.8673628188205199462e0_f64 * t2131 * t2132 * t633 * t3644;
    let t33274 = t2138 * t2132 * t2217 * t879;
    (t33262, t33264, t33266, t33271, t33274)
}
