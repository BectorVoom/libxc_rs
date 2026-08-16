//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2551/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2551(t1041: f64, t4589: f64, t49850: f64, t10969: f64, t41687: f64, t1009: f64, t13939: f64, t1011: f64, t1019: f64, t10868: f64, t248: f64, t4347: f64) -> (f64, f64, f64, f64, f64) {
    let t49852 = t1041 * t49850 * t4589;
    let t49854 = t10969 * t41687;
    let t49864 = t13939 * t1009;
    let t49866 = t49864 * t1011 * t1019;
    let t49871 = t1041 * t248 * t10868 * t4347;
    (t49852, t49854, t49864, t49866, t49871)
}
