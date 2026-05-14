//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 412/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk412<F: Float>(t2014: F, t2035: F, t118: F, t1932: F, t1939: F, t2007: F, t2011: F, t508: F, t569: F, t3: F) -> (F, F, F, F) {
    let t2036 = t2014 * t2035;
    let t2037 = -t118 * t2007 - t1932 * t508 + t2011 * t569 - t1939 + t2036;
    let t2038 = t3 * t2037;
    let t2040 = param_d * t2037;
    (t2036, t2037, t2038, t2040)
}
