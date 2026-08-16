//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1211/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1211(t25: f64, t12061: f64, t1408: f64, t2: f64, t3664: f64, t584: f64, t606: f64, t16: f64, t2249: f64, t3665: f64, t5134: f64, t5137: f64, t514: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t15937 = t12061 * t1408;
    let t15940 = t3664 * t2;
    let t15941 = t584 * t606;
    let t15951 = piecewise3(t26, 0.0_f64, -8.0_f64 / 27.0_f64 * t15937 * t3665 + 16.0_f64 / 9.0_f64 * t15940 * t15941 + 4.0_f64 / 9.0_f64 * t5134 * t2249 + 8.0_f64 / 3.0_f64 * t514 * t584 - 8.0_f64 * t5137 * t16);
    (t15941, t15951)
}
