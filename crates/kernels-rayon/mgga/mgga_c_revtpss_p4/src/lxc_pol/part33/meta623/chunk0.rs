//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2062/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2062(t25331: f64, t27216: f64, t212: f64, t27265: f64, t689: f64, t780: f64, t1568: f64, t7063: f64, t25410: f64, t25413: f64, t27299: f64, t93281: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98825 = t27216 * t25331;
    let t98830 = 0.10975748638225852664e-1_f64 * t689 * t212 * t27265 * t780;
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98851 = 0.25702851531048074406e-1_f64 * t98849 * t25413;
    let t98852 = t27299 * t689;
    let t98853 = t93281 * t98852;
    (t98825, t98830, t98848, t98849, t98851, t98852, t98853)
}
