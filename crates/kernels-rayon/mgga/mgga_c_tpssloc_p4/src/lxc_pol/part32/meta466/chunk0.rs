//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1756/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1756(t1235: f64, t7284: f64, t1240: f64, t1251: f64, t2122: f64, t1170: f64, t7295: f64, t2121: f64, t461: f64, t6729: f64, t7324: f64, t2131: f64, t23508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24633 = t7284 * t1235;
    let t24637 = t1240 * t1251;
    let t24638 = t2122 * t24637;
    let t24645 = t1170 * t7295;
    let t24646 = t2121 * t24645;
    let t24649 = t6729 * t461;
    let t24650 = t7324 * t24649;
    let t24658 = t2131 * t23508;
    (t24633, t24637, t24638, t24645, t24646, t24649, t24650, t24658)
}
