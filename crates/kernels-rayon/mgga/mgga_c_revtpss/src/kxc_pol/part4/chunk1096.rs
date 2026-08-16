//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1096/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1096(t2349: f64, t97: f64, t2255: f64, t658: f64, t2256: f64, t4269: f64, t100: f64, t580: f64, t22: f64, t4273: f64, t10241: f64, t1509: f64, t2358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13475 = t97 * t2349;
    let t13476 = t2255 * t658;
    let t13479 = t4269 * t2256;
    let t13482 = t100 * t580;
    let t13485 = t4273 * t22;
    let t13493 = t10241 * t1509 * t2358;
    (t13475, t13476, t13479, t13482, t13485, t13493)
}
