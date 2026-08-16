//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3191/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3191(t17583: f64, t3172: f64, t3711: f64, t127: f64, t17693: f64, t17695: f64, t5268: f64, t17708: f64, t45779: f64, t13089: f64, t5391: f64, t13085: f64, t5381: f64) -> (f64, f64, f64, f64, f64) {
    let t59386 = t3711 * t3172 * t17583;
    let t59391 = t17693 * t127 * t5268 * t17695;
    let t59401 = t45779 * t17708;
    let t59404 = t5391 * t13089;
    let t59406 = t5381 * t13085;
    (t59386, t59391, t59401, t59404, t59406)
}
