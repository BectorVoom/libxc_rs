//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1800/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1800(t1548: f64, t4343: f64, t800: f64, t10811: f64, t6037: f64, t18444: f64, t4364: f64, t4366: f64, t10846: f64, t10885: f64, t10888: f64, t10891: f64, t10900: f64, t18491: f64, t18495: f64, t18500: f64, t18507: f64, t18511: f64, t2730: f64, t4362: f64, t851: f64) -> (f64, f64, f64, f64) {
    let t18515 = t800 * t1548 * t4343;
    let t18518 = t10811 * t6037;
    let t18521 = t4364 * t18444 * t4366;
    let t18524 = 0.10003937560882938627e-2_f64 * t18491 - 0.25724410870841842183e-1_f64 * t851 * t18495 + 0.85748036236139473944e-2_f64 * t851 * t18500 + 0.13552000749142754193e-3_f64 * t10846 - t10885 + 0.10164000561857065645e-4_f64 * t10888 - 35.0_f64 / 216.0_f64 * t10891 + t2730 * t18507 / 16.0_f64 - t10900 * t18511 / 4.0_f64 + t2730 * t18515 / 8.0_f64 - 0.80031500487063509015e-2_f64 * t18518 + 0.42874018118069736972e-3_f64 * t4362 * t18521;
    (t18515, t18518, t18521, t18524)
}
