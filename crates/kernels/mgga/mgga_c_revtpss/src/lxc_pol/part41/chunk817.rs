//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 817/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk817<F: Float>(t3: F, t5789: F, t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t2230: F, t2233: F, t2235: F, t2239: F) -> (F, F, F, F, F, F, F) {
    let t5790 = t3 * t5789;
    let t5795 = param_d * t5789;
    let t5801 = t116 * t1518;
    let t5802 = t5801 * t670;
    let t5805 = t117 * t4292;
    let t5808 = 3.0 * t1459 * t1918 + 3.0 * t1461 * t1916 + 6.0 * t572 * t5802 + 3.0 * t572 * t5805 + t573 * t5795;
    let t5812 = t2219 + t2221 + t2223 + t2226 + t2228 + t2230 + t2233 + t2235 + t2239;
    (t5790, t5795, t5801, t5802, t5805, t5808, t5812)
}
