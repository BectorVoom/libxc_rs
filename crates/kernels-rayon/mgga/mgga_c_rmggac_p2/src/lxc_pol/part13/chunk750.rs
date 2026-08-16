//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 750/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk750(t2084: f64, t2134: f64, t27: f64, t833: f64, t1180: f64, t673: f64, t1182: f64, t128: f64, t118: f64, t1184: f64, t1986: f64, t7487: f64, t7757: f64) -> (f64, f64, f64, f64, f64) {
    let t35188 = t2134 * t27 * t2084 * t833;
    let t35190 = t1180 * t673;
    let t35192 = t128 * t1182;
    let t35195 = t1986 * t118 * t35192 * t1184;
    let t35204 = t7487 * t7757;
    (t35188, t35190, t35192, t35195, t35204)
}
