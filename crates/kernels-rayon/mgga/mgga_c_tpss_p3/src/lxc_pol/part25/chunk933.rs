//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 933/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk933(t3724: f64, t823: f64, t3762: f64, t845: f64, t1411: f64, t2530: f64, t1408: f64, t2193: f64) -> (f64, f64, f64, f64) {
    let t10923 = t3724 * t823;
    let t10961 = t3762 * t845;
    let t10966 = t1411 * t2530;
    let t10980 = t2193 * t1408;
    (t10923, t10961, t10966, t10980)
}
