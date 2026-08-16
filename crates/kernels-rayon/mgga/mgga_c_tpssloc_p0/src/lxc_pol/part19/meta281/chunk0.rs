//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1046/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1046(t12167: f64, t550: f64, t1380: f64, t1372: f64, t3787: f64, t3793: f64, t1351: f64, t3791: f64, t3856: f64, t3901: f64, t215: f64, t535: f64, t9569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12168 = t12167 * t550;
    let t12169 = t1380 * t12168;
    let t12171 = t3787 * t1372;
    let t12172 = t12171 * t3793;
    let t12177 = t3791 * t1351;
    let t12178 = t12177 * t550;
    let t12179 = t1380 * t12178;
    let t12181 = t3901 * t3856;
    let t12188 = 0.28086419753086419752e-1_f64 * t9569 * t535 * t215;
    (t12168, t12169, t12171, t12172, t12177, t12178, t12179, t12181, t12188)
}
