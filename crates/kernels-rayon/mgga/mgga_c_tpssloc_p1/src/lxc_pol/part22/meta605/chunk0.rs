//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2129/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2129(t1041: f64, t4589: f64, t49850: f64, t10969: f64, t41687: f64, t10868: f64, t248: f64, t4347: f64, t10224: f64, t4343: f64, t973: f64, t3130: f64, t4595: f64) -> (f64, f64, f64, f64, f64) {
    let t49852 = t1041 * t49850 * t4589;
    let t49853 = 5.0_f64 / 20736.0_f64 * t49852;
    let t49854 = t10969 * t41687;
    let t49871 = t1041 * t248 * t10868 * t4347;
    let t49872 = t49871 / 6912.0_f64;
    let t49906 = t973 * t10224 * t4343;
    let t49907 = t49906 / 216.0_f64;
    let t49922 = t3130 * t49850 * t4595;
    (t49853, t49854, t49872, t49907, t49922)
}
