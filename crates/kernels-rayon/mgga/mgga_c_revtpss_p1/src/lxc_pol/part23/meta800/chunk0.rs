//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2626/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2626(t18615: f64, t231: f64, t243: f64, t2661: f64, t2662: f64, t14923: f64, t18478: f64, t10811: f64, t18334: f64, t18629: f64, t10777: f64, t10779: f64, t14671: f64, t18637: f64) -> (f64, f64, f64, f64, f64) {
    let t62458 = t2661 * t2662 * t243 * t18615 * t231;
    let t62460 = t14923 * t18478;
    let t62475 = t10811 * t18334;
    let t62494 = t10811 * t18629;
    let t62498 = t10777 * t10779 * t14671 * t18637;
    (t62458, t62460, t62475, t62494, t62498)
}
