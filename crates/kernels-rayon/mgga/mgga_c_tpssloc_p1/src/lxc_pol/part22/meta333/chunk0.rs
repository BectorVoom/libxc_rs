//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1524/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1524(t12345: f64, t1831: f64, t1362: f64, t16060: f64, t12339: f64, t3866: f64, t5314: f64, t3865: f64, t5234: f64) -> (f64, f64, f64, f64, f64) {
    let t16317 = t12345 * t1831;
    let t16321 = t16060 * t1362;
    let t16325 = 7.0_f64 / 576.0_f64 * t12339 * t1831;
    let t16331 = 7.0_f64 / 576.0_f64 * t3866 * t5314;
    let t16336 = t5234 * t3865;
    (t16317, t16321, t16325, t16331, t16336)
}
