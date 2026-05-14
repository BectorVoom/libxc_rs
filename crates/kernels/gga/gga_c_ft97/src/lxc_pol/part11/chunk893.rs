//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 893/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk893<F: Float>(t8392: F, t9359: F, t2133: F, t582: F, t1559: F, t2075: F, t2157: F, t9124: F, t2214: F, t38953: F, t9136: F, t9363: F, t9118: F, t1570: F, t2178: F, t2180: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t40733 = t8392 * t9359;
    let t40735 = t582 * t2133;
    let t40739 = t1559 * t2075;
    let t40744 = t1559 * t2157;
    let t40749 = t8392 * t9124;
    let t40751 = t38953 * t2214;
    let t40753 = t8392 * t9136;
    let t40755 = t8392 * t9363;
    let t40757 = t8392 * t9118;
    let t40759 = t2178 * t1570;
    let t40760 = t1559 * t2180;
    (t40733, t40735, t40739, t40744, t40749, t40751, t40753, t40755, t40757, t40759, t40760)
}
