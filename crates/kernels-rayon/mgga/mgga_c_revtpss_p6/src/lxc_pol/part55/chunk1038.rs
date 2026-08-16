//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1038/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1038(t2056: f64, t32392: f64, t7002: f64, t94: f64, t7367: f64, t8634: f64, t2322: f64, t8641: f64, t4254: f64, t1936: f64, t7474: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32393 = t32392 * t2056;
    let t32394 = t94 * t7002;
    let t32395 = t32394 * t2056;
    let t32396 = t8634 * t7367;
    let t32397 = t2322 * t8641;
    let t32398 = t4254 * t8641;
    let t32401 = t7474 * t1936;
    let t32402 = t651 * t32401;
    (t32393, t32394, t32395, t32396, t32397, t32398, t32401, t32402)
}
