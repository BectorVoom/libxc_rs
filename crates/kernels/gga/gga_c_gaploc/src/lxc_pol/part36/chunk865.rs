//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 865/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk865<F: Float>(t12971: F, t12976: F, t12979: F, t12983: F, t1445: F, t1562: F, t1580: F, t1599: F, t1641: F, t1646: F, t40332: F, t40336: F, t42064: F, t42067: F, t42069: F, t42072: F, t42074: F, t42077: F, t42081: F, t42086: F, t42092: F, t42093: F, t42099: F, t42123: F, t42130: F, t42138: F, t475: F, t528: F, t531: F, t557: F, t567: F, t568: F, t569: F, t574: F) -> F {
    let t42142 = -t42064 + t42067 - t42069 + t42072 + F::new(0.14300195980740170668e1) * t42074 - F::new(0.12269736305254639897e2) * t42077 - t42081 - F::new(0.35750489951850426669e0) * t528 * t12971 * t1646 + F::new(0.23005755572352449806e1) * t567 * t1445 * t42086 + t42092 - F::new(0.69017266717057349418e1) * t1562 * t1445 * t42093 * t475 - t42099 - F::new(0.23005755572352449806e1) * t1641 * t12983 - F::new(0.23005755572352449806e1) * t574 * t568 * t569 * t42123 - F::new(0.35750489951850426669e0) * t1599 * t12979 - F::new(0.35750489951850426669e0) * t557 * t531 * t42130 + F::new(0.23005755572352449806e1) * t1580 * t12976 + F::new(0.21450293971110256002e1) * t42138 - F::new(0.1533717038156829987e1) * t40332 - F::new(0.38342925953920749676e0) * t40336;
    t42142
}
