//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2109/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2109(t18446: f64, t25270: f64, t18629: f64, t18428: f64, t27261: f64, t18651: f64, t18639: f64, t105985: f64, t105987: f64, t105989: f64, t105991: f64, t105993: f64, t98937: f64, t98950: f64) -> f64 {
    let t105995 = t25270 * t18446;
    let t105997 = t25270 * t18629;
    let t105999 = t27261 * t18428;
    let t106001 = t25270 * t18651;
    let t106003 = t25270 * t18639;
    let t106005 = -0.80031500487063509016e-2_f64 * t98937 + 0.17149607247227894789e-2_f64 * t105985 - t98950 - 0.85748036236139473944e-3_f64 * t105987 + 0.34299214494455789578e-2_f64 * t105989 - 0.25724410870841842183e-2_f64 * t105991 - 0.85748036236139473945e-2_f64 * t105993 + 0.17149607247227894789e-2_f64 * t105995 + 0.17149607247227894789e-2_f64 * t105997 - 0.34299214494455789578e-2_f64 * t105999 - 0.42874018118069736972e-3_f64 * t106001 + 0.34299214494455789578e-2_f64 * t106003;
    t106005
}
