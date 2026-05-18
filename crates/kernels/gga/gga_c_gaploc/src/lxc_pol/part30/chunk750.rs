//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 750/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk750<F: Float>(t5396: F, t7069: F, t286: F, t6361: F, t708: F, t1687: F, t6365: F, t5337: F, t5340: F, t6372: F, t5345: F, t5348: F) -> (F, F, F, F, F) {
    let t7070 = t5396 * t7069;
    let t7088 = t6361 * t286 * t708;
    let t7090 = t6365 * t1687;
    let t7093 = t6372 * t5337 * t5340;
    let t7096 = t5345 * t6372 * t5348;
    (t7070, t7088, t7090, t7093, t7096)
}
