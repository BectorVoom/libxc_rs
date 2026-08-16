//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1152/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1152(t23109: f64, t2632: f64, t81914: f64, t98779: f64, t23110: f64, t232: f64, t236: f64, t5611: f64, t5587: f64, t81886: f64, t23041: f64, t5619: f64) -> (f64, f64, f64, f64, f64) {
    let t98782 = t23109 * t81914 * t98779 * t2632;
    let t98787 = t23109 * t23110 * t236 * t5611 * t232;
    let t98791 = t23109 * t23110 * t98779 * t232;
    let t98796 = t81886 * t5587;
    let t98798 = t23041 * t5619;
    (t98782, t98787, t98791, t98796, t98798)
}
