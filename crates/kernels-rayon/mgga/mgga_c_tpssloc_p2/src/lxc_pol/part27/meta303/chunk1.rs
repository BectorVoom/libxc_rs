//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1366/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1366(t10475: f64, t10478: f64, t10472: f64, t3131: f64, t360: f64, t248: f64, t2776: f64, t3051: f64, t1041: f64, t3103: f64, t3109: f64, t3114: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10489 = t248 * t3051 * t2776;
    let t10490 = t1041 * t10489;
    let t10496 = t3109 * t3103;
    let t10504 = t3114 * t3103;
    (t10480, t10482, t10489, t10490, t10496, t10504)
}
