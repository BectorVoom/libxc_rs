//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 695/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk695<F: Float>(t10022: F, t10089: F, t10162: F, t9843: F, t258: F, t9974: F, t10003: F, t10054: F, t10122: F, t10150: F, t10154: F, t2331: F, t2465: F, t247: F, t2617: F, t263: F, t719: F, t771: F, t9512: F, t9514: F, t9781: F, t9839: F) -> (F, F, F) {
    let t10164 = t9843 + t10022 + t10089 + t10162;
    let t10166 = t9974 * t258;
    let t10174 = -t10164 * t247 - 3.0 * t2331 * t771 - 3.0 * t2465 * t771 - 3.0 * t2617 * t719 - t263 * t9512 - 2.0 * t263 * t9514 - t263 * t9781 + 12.0 * t10003 - 12.0 * t10054 - 2.0 * t10122 - 6.0 * t10150 - 6.0 * t10154 + 2.0 * t10166 + 12.0 * t9839;
    (t10164, t10166, t10174)
}
