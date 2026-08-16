//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1060/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1060(t241: f64, t67: f64, t6924: f64, t12156: f64, t820: f64, t3866: f64, t3872: f64, t3876: f64, t12012: f64, t1367: f64, t1339: f64, t2690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12351 = t241 * t6924 * t67;
    let t12353 = t12351 * t820 * t12156;
    let t12356 = t3866 * t3872;
    let t12358 = t3866 * t3876;
    let t12361 = t1367 * t820 * t12012;
    let t12364 = t1339 * t2690;
    (t12351, t12353, t12356, t12358, t12361, t12364)
}
