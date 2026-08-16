//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1066/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1066(t121134: f64, t121365: f64, t32296: f64, t531: f64, t25081: f64, t8763: f64, t33553: f64, t575: f64, t1464: f64, t8970: f64, t136: f64, t33362: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121366 = t121365 * t121134;
    let t121441 = t531 * t32296;
    let t122820 = t8763 * t25081;
    let t124440 = t33553 * t575;
    let t124442 = t8970 * t1464;
    let t124455 = t33362 * t136;
    (t121366, t121441, t122820, t124440, t124442, t124455)
}
