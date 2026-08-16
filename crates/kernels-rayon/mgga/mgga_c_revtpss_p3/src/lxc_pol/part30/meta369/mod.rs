//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta369(t127: f64, t3661: f64, t371: f64, t1235: f64, t12640: f64, t225: f64, t12657: f64, t480: f64, t3667: f64, t3678: f64, t1236: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12963, t12964, t12966, t12975, t12976, t12979, t12984) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1396(t127, t3661, t371, t1235, t12640, t225, t12657, t480, t3667, t3678, t1236, t676);
    (t12963, t12964, t12966, t12975, t12976, t12979, t12984)
}
