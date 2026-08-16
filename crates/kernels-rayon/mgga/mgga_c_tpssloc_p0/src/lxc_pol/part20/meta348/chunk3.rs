//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1653/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1653(t12168: f64, t1380: f64, t1372: f64, t3787: f64, t3793: f64, t1351: f64, t3791: f64, t550: f64) -> (f64, f64, f64, f64) {
    let t12169 = t1380 * t12168;
    let t12171 = t3787 * t1372;
    let t12172 = t12171 * t3793;
    let t12177 = t3791 * t1351;
    let t12178 = t12177 * t550;
    (t12169, t12172, t12177, t12178)
}
