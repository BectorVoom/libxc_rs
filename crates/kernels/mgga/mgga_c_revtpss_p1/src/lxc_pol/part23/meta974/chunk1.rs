//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3311/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3311<F: Float>(t1868: F, t5778: F, t22809: F, t566: F, t1353: F, t1448: F, t1450: F, t198: F, t22813: F, t4139: F, t47113: F, t47116: F, t47118: F, t47122: F, t47124: F, t5536: F, t5542: F, t85987: F, t85989: F, t85990: F) -> F {
    let t86815 = t1868 * t5778;
    let t86819 = t566 * t22809;
    let t86823 = F::cast_from(6.0_f64) * t1448 * t1450 * t198 * t22813 + F::cast_from(6.0_f64) * t1353 * t5536 * t86819 - F::cast_from(18.0_f64) * t4139 * t5542 * t86815 + t47113 + t47116 - t47118 + t47122 + t47124 - t85987 + t85989 + t85990;
    t86823
}
