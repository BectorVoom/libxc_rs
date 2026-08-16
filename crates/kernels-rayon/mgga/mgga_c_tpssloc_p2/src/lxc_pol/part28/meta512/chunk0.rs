//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1760/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1760(t1799: f64, t3850: f64, t1824: f64, t3791: f64, t16028: f64, t225: f64, t1372: f64, t5286: f64, t3879: f64, t16205: f64, t562: f64, t1834: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54165 = t1799 * t3850;
    let t54258 = t1824 * t3791;
    let t54825 = t16028 * t225;
    let t54840 = t1372 * t5286;
    let t54854 = t3879 * t1824;
    let t54883 = t562 * t16205;
    let t54918 = t1834 * t3850;
    (t54165, t54258, t54825, t54840, t54854, t54883, t54918)
}
