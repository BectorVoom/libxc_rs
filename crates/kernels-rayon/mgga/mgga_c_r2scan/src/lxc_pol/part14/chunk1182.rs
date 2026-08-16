//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1182/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1182(t37031: f64, t8367: f64, t3366: f64, t8355: f64, t23495: f64, t3363: f64, t1035: f64, t1339: f64, t352: f64, t1343: f64, t3675: f64, t12025: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40842 = t37031 * t8367;
    let t40844 = t8355 * t3366;
    let t40848 = t23495 * t3363;
    let t41058 = t1035 * t1339 * t352;
    let t41065 = t3675 * t1343;
    let t41116 = 45.0_f64 / 32.0_f64 * t12025;
    (t40842, t40844, t40848, t41058, t41065, t41116)
}
