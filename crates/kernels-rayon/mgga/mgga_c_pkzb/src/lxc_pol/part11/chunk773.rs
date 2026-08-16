//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 773/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk773(t2648: f64, t6966: f64, t164: f64, t2639: f64, t1041: f64, t5296: f64, t177: f64, t5305: f64, t1037: f64, t5384: f64, t1769: f64, t2663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6968 = 0.20007875121765877254e-2_f64 * t6966 * t2648;
    let t6970 = t2639 * t164;
    let t6988 = t5296 * t1041;
    let t6990 = t5305 * t177;
    let t6995 = t5384 * t1037;
    let t6998 = 0.40015750243531754508e-1_f64 * t1769 * t2663;
    (t6968, t6970, t6988, t6990, t6995, t6998)
}
