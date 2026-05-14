//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 428/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk428<F: Float>(t1450: F, t2034: F, t2014: F, t118: F, t1932: F, t1939: F, t2007: F, t2011: F, t508: F, t569: F, t3: F, t117: F, t1936: F, t572: F, t573: F, t10: F, t17: F) -> (F, F, F, F, F, F, F) {
    let t2035 = t2034 * t1450;
    let t2036 = t2014 * t2035;
    let t2037 = -t118 * t2007 - t1932 * t508 + t2011 * t569 - t1939 + t2036;
    let t2038 = t3 * t2037;
    let t2040 = param_d * t2037;
    let t2042 = t117 * t1936;
    let t2044 = 3.0 * t572 * t2042;
    let t2045 = t2040 * t573 + t2044;
    let t2219 = 2.0 * t10 * t17;
    (t2035, t2037, t2038, t2040, t2042, t2045, t2219)
}
