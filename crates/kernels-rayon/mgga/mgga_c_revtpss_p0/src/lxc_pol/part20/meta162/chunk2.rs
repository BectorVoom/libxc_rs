//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 874/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk874(t213: f64, t5744: f64, t4086: f64, t640: f64, t76: f64, t159: f64, t793: f64, t1448: f64, t4147: f64, t587: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t6977 = t76 * t640;
    let t7021 = t793 * t159;
    let t7315 = t4147 * t1448;
    let t8779 = 1.0_f64 / t65 / t587;
    (t5745, t5755, t6977, t7021, t7315, t8779)
}
