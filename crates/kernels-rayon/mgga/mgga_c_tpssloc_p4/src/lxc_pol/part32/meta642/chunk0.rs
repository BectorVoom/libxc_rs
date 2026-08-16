//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2060/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2060(t90472: f64, t1799: f64, t3886: f64, t1887: f64, t80827: f64, t26334: f64, t26339: f64, t81159: f64, t22716: f64, t7697: f64, t26216: f64, t26210: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t90473 = 0.76763589786250567036e-1_f64 * t90472;
    let t90488 = t3886 * t1799;
    let t90497 = t80827 * t1887;
    let t90498 = t90497 * t26334;
    let t90500 = t81159 * t26339;
    let t90501 = 0.76763589786250567036e-1_f64 * t90500;
    let t90503 = t22716 * t7697;
    let t90511 = t81159 * t26216;
    let t90512 = 0.76763589786250567036e-1_f64 * t90511;
    let t90514 = t6897 * t794 * t26210;
    (t90473, t90488, t90497, t90498, t90501, t90503, t90512, t90514)
}
