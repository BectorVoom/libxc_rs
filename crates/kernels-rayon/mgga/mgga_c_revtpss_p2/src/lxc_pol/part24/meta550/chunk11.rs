//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1637/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1637(t18865: f64, t1940: f64, t198: f64, t23279: f64, t2403: f64, t29598: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t4541: f64, t4546: f64, t5962: f64, t6079: f64, t61033: f64, t77333: f64, t87670: f64, t87671: f64, t87673: f64, t87674: f64, t87675: f64) -> f64 {
    let t87966 = -36.0_f64 * t18865 * t2403 * t29598 + 12.0_f64 * t1940 * t6079 * t61033 + 36.0_f64 * t198 * t5962 * t77333 + 72.0_f64 * t23279 * t4541 * t4546 + t40067 - t40072 + t40167 - t40171 - t40184 - t87670 - t87671 + t87673 - t87674 + t87675;
    t87966
}
