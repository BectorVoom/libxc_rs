//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1604/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1604(t87280: f64, t87292: f64, t162: f64, t187: f64, t150: f64, t190: f64, t18850: f64, t2403: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39442: f64, t5962: f64, t87262: f64, t87263: f64, t87265: f64, t87267: f64, t87268: f64) -> (f64, f64, f64) {
    let t87293 = t87280 + t87292;
    let t87296 = 0.19751673498613801407e-1_f64 * t87293 * t162 * t187;
    let t87298 = t150 * t87293 * t190;
    let t87302 = 18.0_f64 * t18850 * t2403 * t5962 - t39419 - t39422 - t39429 - t39432 + t39442 + t87262 + t87263 + t87265 + t87267 - t87268 + t87296 + t87298;
    (t87296, t87298, t87302)
}
