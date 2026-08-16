//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1705/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1705(t12345: f64, t1369: f64, t241: f64, t67: f64, t6924: f64, t3866: f64, t3872: f64, t3876: f64, t1339: f64, t2690: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12346 = t12345 * t1369;
    let t12351 = t241 * t6924 * t67;
    let t12356 = t3866 * t3872;
    let t12358 = t3866 * t3876;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    (t12346, t12351, t12356, t12358, t12364, t12365)
}
