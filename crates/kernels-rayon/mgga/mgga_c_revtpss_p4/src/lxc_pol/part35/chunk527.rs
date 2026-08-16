//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 527/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk527(t213: f64, t5744: f64, t4086: f64, t1892: f64, t545: f64, t869: f64, t689: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t116: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t5759 = t545 * t1892;
    let t5760 = t869 * t5759;
    let t5761 = t689 * t5760;
    let t5763 = t1892 * t72;
    let t5765 = t1432 * t5763 * t686;
    let t5767 = t1385 * t1892;
    let t5801 = t116 * t1518;
    (t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5801)
}
