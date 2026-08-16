//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2430/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2430<F: Float>(t1548: F, t4343: F, t800: F, t10811: F, t6037: F, t18444: F, t4364: F, t4366: F, t10846: F, t10885: F, t10888: F, t10891: F, t10900: F, t18491: F, t18495: F, t18500: F, t18507: F, t18511: F, t2730: F, t4362: F, t851: F) -> (F, F, F) {
    let t18515 = t800 * t1548 * t4343;
    let t18518 = t10811 * t6037;
    let t18521 = t4364 * t18444 * t4366;
    let t18524 = F::cast_from(0.10003937560882938627e-2_f64) * t18491 - F::cast_from(0.25724410870841842183e-1_f64) * t851 * t18495 + F::cast_from(0.85748036236139473944e-2_f64) * t851 * t18500 + F::cast_from(0.13552000749142754193e-3_f64) * t10846 - t10885 + F::cast_from(0.10164000561857065645e-4_f64) * t10888 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t10891 + t2730 * t18507 / F::cast_from(16.0_f64) - t10900 * t18511 / F::cast_from(4.0_f64) + t2730 * t18515 / F::cast_from(8.0_f64) - F::cast_from(0.80031500487063509015e-2_f64) * t18518 + F::cast_from(0.42874018118069736972e-3_f64) * t4362 * t18521;
    (t18515, t18521, t18524)
}
