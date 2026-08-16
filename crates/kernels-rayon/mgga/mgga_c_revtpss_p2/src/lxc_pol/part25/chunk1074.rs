//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1074/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1074(t1261: f64, t12886: f64, t12690: f64, t225: f64, t480: f64, t1231: f64, t3655: f64, t1256: f64, t3651: f64, t2434: f64, t371: f64, t482: f64) -> (f64, f64, f64, f64, f64) {
    let t12887 = t1261 * t12886;
    let t12889 = t12690 * t225;
    let t12890 = t12889 * t480;
    let t12893 = t1231 * t3655;
    let t12895 = t3651 * t1256;
    let t12898 = t371 * t2434 * t482;
    (t12887, t12890, t12893, t12895, t12898)
}
