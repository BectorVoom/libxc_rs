//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1257/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1257<F: Float>(t29142: F, t8392: F, t29138: F, t29256: F, t1882: F, t29363: F, t112432: F, t112582: F, t112627: F, t15162: F, t15259: F, t15477: F, t1901: F, t24886: F, t29051: F, t29056: F, t29071: F, t296: F, t446: F, t56127: F, t56815: F, t6353: F, t6360: F, t840: F, t98944: F, t98957: F, t98960: F) -> (F,) {
    let t113807 = 4.0 / 9.0 * t8392 * t29142;
    let t113809 = 4.0 / 9.0 * t8392 * t29138;
    let t113816 = 2.0 / 27.0 * t8392 * t29256;
    let t113831 = 2.0 / 9.0 * t1882 * t29363;
    let t113840 = -2.0 / 9.0 * t1901 * t24886 * t15259 + t113807 + t113809 + 2.0 * t1901 * t29071 * t6360 * t15477 + 4.0 / 27.0 * t98944 - t113816 + 4.0 / 3.0 * t446 * t296 * t112432 + 4.0 / 3.0 * t446 * t296 * t112582 + 2.0 / 3.0 * t446 * t296 * t112627 + t446 * t840 * t6353 * t15162 / 3.0 - t113831 + 2.0 / 27.0 * t98957 + 4.0 / 27.0 * t98960 - 4.0 / 3.0 * t1901 * t56815 * t29056 - 4.0 / 3.0 * t1901 * t56127 * t29051;
    (t113840,)
}
