//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1311/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1311(t10069: f64, t10934: f64, t10518: f64, t10542: f64, t39419: f64, t39422: f64, t39424: f64, t39426: f64, t39429: f64, t39432: f64, t39434: f64, t39437: f64, t39439: f64, t39442: f64, t39483: f64) -> (f64, f64, f64) {
    let t39726 = t10069 * t10934;
    let t39731 = t10542 * t10518;
    let t39736 = -t39419 - t39422 - t39424 - t39426 - t39429 - t39432 + t39434 + t39437 + t39439 + t39442 - t39483;
    (t39726, t39731, t39736)
}
