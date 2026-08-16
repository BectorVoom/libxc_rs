//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1301/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1301(t9864: f64, t9866: f64, t3966: f64, t751: f64, t707: f64, t2379: f64, t262: f64, t157: f64, t9897: f64, t2244: f64, t4195: f64, t2371: f64, t4199: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12927 = 0.23392894490538584828e1_f64 * t9864;
    let t12928 = 0.34631718211362927518e2_f64 * t9866;
    let t12932 = t751 * t3966;
    let t12934 = 8.0_f64 * t707 * t12932;
    let t12935 = t2379 * t262;
    let t12939 = t9897 * t157;
    let t12940 = t4195 * t2244;
    let t12942 = 24.0_f64 * t12939 * t12940;
    let t12943 = t4199 * t2371;
    (t12927, t12928, t12934, t12935, t12942, t12943)
}
