//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1037/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1037(t1936: f64, t7474: f64, t651: f64, t7374: f64, t8634: f64, t2055: f64, t7221: f64, t649: f64, t8686: f64, t1937: f64, t26399: f64, t28658: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32401 = t7474 * t1936;
    let t32402 = t651 * t32401;
    let t32404 = t8634 * t7374;
    let t32410 = t7221 * t2055;
    let t32415 = t649 * t8686;
    let t32417 = 2.0_f64 * t26399 * t1937;
    let t32419 = 2.0_f64 * t28658 * t1937;
    (t32401, t32402, t32404, t32410, t32415, t32417, t32419)
}
