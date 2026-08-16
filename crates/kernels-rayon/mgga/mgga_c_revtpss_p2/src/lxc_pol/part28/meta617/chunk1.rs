//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2159/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2159(t98949: f64, t14788: f64, t25270: f64, t14682: f64, t14804: f64, t27261: f64, t14793: f64, t92952: f64, t92956: f64, t98937: f64, t98940: f64, t98943: f64, t98945: f64, t98947: f64) -> f64 {
    let t98950 = 0.16006300097412701803e-1_f64 * t98949;
    let t98951 = t25270 * t14788;
    let t98953 = t25270 * t14682;
    let t98955 = t27261 * t14804;
    let t98957 = t25270 * t14793;
    let t98959 = -0.80031500487063509015e-2_f64 * t98937 - 0.16006300097412701803e-1_f64 * t92952 - 0.42874018118069736972e-3_f64 * t98940 + 0.2032800112371413129e-3_f64 * t92956 - 0.85748036236139473945e-2_f64 * t98943 + 0.17149607247227894789e-2_f64 * t98945 - 0.34299214494455789578e-2_f64 * t98947 - t98950 - 0.17149607247227894789e-1_f64 * t98951 - 0.42874018118069736972e-3_f64 * t98953 - 0.68598428988911579156e-2_f64 * t98955 + 0.34299214494455789578e-2_f64 * t98957;
    t98959
}
