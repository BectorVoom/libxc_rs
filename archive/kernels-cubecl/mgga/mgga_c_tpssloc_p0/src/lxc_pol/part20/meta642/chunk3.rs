//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2353/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2353<F: Float>(t10186: F, t13780: F, t13785: F, t13839: F, t2986: F, t42837: F, t10236: F, t12652: F, t10913: F, t13554: F, t13536: F, t12648: F) -> (F, F, F, F, F, F, F) {
    let t48242 = t10186 * t13780;
    let t48244 = t10186 * t13785;
    let t48250 = t2986 * t42837 * t13839;
    let t48256 = t10236 * t12652;
    let t48260 = t13554 * t10913;
    let t48265 = t13536 * t10913;
    let t48269 = t10236 * t12648;
    (t48242, t48244, t48250, t48256, t48260, t48265, t48269)
}
