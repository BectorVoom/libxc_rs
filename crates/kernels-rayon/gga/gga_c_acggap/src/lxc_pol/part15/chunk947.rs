//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 947/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk947(t3919: f64, t8347: f64, t29991: f64, t639: f64, t7987: f64, t8104: f64, t2131: f64, t2132: f64, t3644: f64, t633: f64, t2138: f64, t2217: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t33250 = t8347 * t3919;
    let t33256 = t29991 * t639;
    let t33266 = t7987 * t8104;
    let t33271 = 0.8673628188205199462e0_f64 * t2131 * t2132 * t633 * t3644;
    let t33274 = t2138 * t2132 * t2217 * t879;
    (t33250, t33256, t33266, t33271, t33274)
}
