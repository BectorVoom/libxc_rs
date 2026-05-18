//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1040/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1040<F: Float>(t11143: F, t11159: F, t11231: F, t11236: F, t11238: F, t11316: F, t11318: F, t11321: F, t11325: F, t11329: F, t11355: F, t11363: F, t11536: F, t135: F, t273: F, t957: F) -> F {
    let t11540 = t11536 * t135 * t273 * t957 + t11143 - t11159 - t11231 + t11236 + t11238 + t11316 + t11318 - t11321 - t11325 + t11329 - t11355 + t11363;
    t11540
}
