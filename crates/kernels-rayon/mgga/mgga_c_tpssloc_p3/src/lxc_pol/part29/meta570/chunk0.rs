//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1987/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1987(t16465: f64, t225: f64, t12250: f64, t1824: f64, t1799: f64, t3791: f64, t3850: f64, t16028: f64, t1372: f64, t5286: f64, t3879: f64, t16205: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t53866 = t16465 * t225;
    let t54014 = t1824 * t12250;
    let t54068 = t1799 * t3791;
    let t54153 = t1824 * t3850;
    let t54165 = t1799 * t3850;
    let t54258 = t1824 * t3791;
    let t54825 = t16028 * t225;
    let t54840 = t1372 * t5286;
    let t54854 = t3879 * t1824;
    let t54883 = t562 * t16205;
    (t53866, t54014, t54068, t54153, t54165, t54258, t54825, t54840, t54854, t54883)
}
