//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 865/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk865(t12971: f64, t12976: f64, t12979: f64, t12983: f64, t1445: f64, t1562: f64, t1580: f64, t1599: f64, t1641: f64, t1646: f64, t40332: f64, t40336: f64, t42064: f64, t42067: f64, t42069: f64, t42072: f64, t42074: f64, t42077: f64, t42081: f64, t42086: f64, t42092: f64, t42093: f64, t42099: f64, t42123: f64, t42130: f64, t42138: f64, t475: f64, t528: f64, t531: f64, t557: f64, t567: f64, t568: f64, t569: f64, t574: f64) -> f64 {
    let t42142 = -t42064 + t42067 - t42069 + t42072 + 0.14300195980740170668e1_f64 * t42074 - 0.12269736305254639897e2_f64 * t42077 - t42081 - 0.35750489951850426669e0_f64 * t528 * t12971 * t1646 + 0.23005755572352449806e1_f64 * t567 * t1445 * t42086 + t42092 - 0.69017266717057349418e1_f64 * t1562 * t1445 * t42093 * t475 - t42099 - 0.23005755572352449806e1_f64 * t1641 * t12983 - 0.23005755572352449806e1_f64 * t574 * t568 * t569 * t42123 - 0.35750489951850426669e0_f64 * t1599 * t12979 - 0.35750489951850426669e0_f64 * t557 * t531 * t42130 + 0.23005755572352449806e1_f64 * t1580 * t12976 + 0.21450293971110256002e1_f64 * t42138 - 0.1533717038156829987e1_f64 * t40332 - 0.38342925953920749676e0_f64 * t40336;
    t42142
}
