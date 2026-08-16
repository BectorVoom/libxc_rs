//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2551/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2551<F: Float>(t1041: F, t4589: F, t49850: F, t10969: F, t41687: F, t1009: F, t13939: F, t1011: F, t1019: F, t10868: F, t248: F, t4347: F) -> (F, F, F, F, F) {
    let t49852 = t1041 * t49850 * t4589;
    let t49854 = t10969 * t41687;
    let t49864 = t13939 * t1009;
    let t49866 = t49864 * t1011 * t1019;
    let t49871 = t1041 * t248 * t10868 * t4347;
    (t49852, t49854, t49864, t49866, t49871)
}
