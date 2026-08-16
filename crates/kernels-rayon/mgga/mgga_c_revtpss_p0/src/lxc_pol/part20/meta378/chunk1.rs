//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1371/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1371(t2430: f64, t836: f64, t10638: f64, t125: f64, t124: f64, t2645: f64, t14686: f64, t14931: f64, t4366: f64, t2722: f64, t10777: f64, t10779: f64, t2749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40560 = t2430 * t836;
    let t40569 = t125 * t10638;
    let t40578 = t124 * t2645;
    let t40581 = t14931 * t14686 * t40578 * t4366;
    let t40583 = t124 * t2722;
    let t40586 = t10777 * t10779 * t40583 * t2749;
    (t40560, t40569, t40578, t40581, t40583, t40586)
}
