//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2110/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2110(t18643: f64, t92955: f64, t18456: f64, t27261: f64, t6037: f64, t92951: f64, t18521: f64, t25222: f64, t6030: f64, t103264: f64, t92963: f64, t92966: f64, t92969: f64, t92976: f64, t98968: f64, t98973: f64) -> f64 {
    let t106006 = t92955 * t18643;
    let t106008 = t27261 * t18456;
    let t106010 = t92951 * t6037;
    let t106012 = t27261 * t18521;
    let t106014 = t25222 * t6030;
    let t106020 = 0.2032800112371413129e-3_f64 * t106006 + 0.25724410870841842183e-2_f64 * t106008 - 0.16006300097412701803e-1_f64 * t106010 + 0.85748036236139473944e-3_f64 * t106012 + 0.80031500487063509015e-2_f64 * t106014 + 0.50820002809285328225e-5_f64 * t92963 - 0.36143185997963725434e-4_f64 * t92966 - 35.0_f64 / 216.0_f64 * t92969 - t103264 - 0.57165357490759649295e-3_f64 * t98968 - t98973 + t92976;
    t106020
}
