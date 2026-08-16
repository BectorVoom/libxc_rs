//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 812/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk812(t462: f64, t8077: f64, t1734: f64, t2144: f64, t1246: f64, t493: f64, t8054: f64, t1244: f64, t1729: f64, t2121: f64, t2149: f64, t2152: f64, t470: f64, t7283: f64, t7361: f64, t7373: f64, t7999: f64, t8067: f64, t8070: f64, t8074: f64) -> (f64, f64, f64, f64, f64) {
    let t8078 = t462 * t8077;
    let t8082 = t2144 * t1734;
    let t8083 = t8082 * t1246;
    let t8085 = t493 * t8054;
    let t8087 = -0.21932454224643019153e-1_f64 * t7999 * t2149 + t7361 - 0.27415567780803773942e-2_f64 * t7283 * t8067 - 0.82246703342411321825e-2_f64 * t7283 * t8070 + 0.82246703342411321825e-2_f64 * t7373 * t8074 + 0.82246703342411321825e-2_f64 * t2121 * t8078 + t1729 * t2152 + t1244 * t8083 + t470 * t8085;
    (t8078, t8082, t8083, t8085, t8087)
}
