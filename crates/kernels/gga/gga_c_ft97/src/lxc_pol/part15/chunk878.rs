//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 878/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk878<F: Float>(t1527: F, t419: F, t85456: F, t37821: F, t58708: F, t58719: F, t74068: F, t74126: F, t74143: F, t74148: F, t74153: F, t74162: F, t85454: F, t1737: F, t420: F, t85451: F) -> (F, F, F) {
    let t85458 = t419 * t1527 * t85456;
    let t85460 = 0.85124811172839506172e-2 * t74162 - t37821 - 0.85124811172839506172e-2 * t58708 - 0.51074886703703703704e-1 * t74126 + 0.34049924469135802468e-1 * t74068 + 0.51074886703703703704e-1 * t74143 + 0.26483274587105624143e-1 * t74148 - 0.68099848938271604939e-1 * t74153 - 0.1134997482304526749e-1 * t58719 - 0.38306165027777777778e-1 * t85454 - 0.51074886703703703704e-1 * t85458;
    let t85463 = t419 * t420 * t1737 * t85451;
    (t85458, t85460, t85463)
}
