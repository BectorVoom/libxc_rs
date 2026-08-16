//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1018/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1018(t45: f64, t4802: f64, t823: f64, t4573: f64, t8050: f64, t2225: f64, t4579: f64, t13335: f64, t3431: f64, t3575: f64, t581: f64, t78: f64, t8061: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t14080 = t4802 * t823;
    let t14084 = t8050 * t4573;
    let t14089 = t2225 * t4579;
    let t14095 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t14084 * t581 + 8.0_f64 / 9.0_f64 * t3575 * t3431 + 4.0_f64 / 9.0_f64 * t14089 * t581 + 4.0_f64 / 3.0_f64 * t78 * t13335);
    let t14096 = t8061 * t4573;
    (t14080, t14095, t14096)
}
