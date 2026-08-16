//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 861/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk861(t265: f64, t393: f64, t1100: f64, t1102: f64, t1699: f64, t198: f64, t25709: f64, t25713: f64, t27708: f64, t27712: f64, t27717: f64, t27754: f64, t336: f64, t5019: f64, t5023: f64, t7181: f64) -> f64 {
    let t394 = t265 < t393;
    let t27755 = piecewise3(t394, t1102 * t198 * t27708 * t336 - t1100 * t27712 * t5023 - t1699 * t25709 * t5023 + 2.0_f64 * t25713 * t27717 * t5023 - t5019 * t5023 * t7181, t27754);
    t27755
}
