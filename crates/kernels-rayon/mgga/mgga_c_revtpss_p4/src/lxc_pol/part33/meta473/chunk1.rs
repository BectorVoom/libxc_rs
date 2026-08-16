//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1722/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1722(t22190: f64, t22203: f64, t22210: f64, t22220: f64, t225: f64, t1877: f64, t73: f64, t4010: f64, t6836: f64, t1353: f64, t5591: f64, t5651: f64) -> (f64, f64, f64, f64) {
    let t22223 = (t22190 + t22203 + t22210 + t22220) * t225;
    let t22229 = t1877 * t73;
    let t22236 = t4010 * t6836;
    let t22237 = t22236 * t1353;
    let t22240 = t5651 * t5591;
    (t22223, t22229, t22237, t22240)
}
