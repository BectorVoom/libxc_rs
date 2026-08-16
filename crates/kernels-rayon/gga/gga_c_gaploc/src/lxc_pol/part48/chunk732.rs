//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 732/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk732(t10530: f64, t1434: f64, t584: f64, t1559: f64, t197: f64, t1563: f64, t202: f64, t4526: f64, t561: f64, t4539: f64, t524: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18372 = t584 * t10530 * t1434;
    let t18535 = t1559 * t197;
    let t18540 = 1.0_f64 / t1563 / t202;
    let t18651 = t561 * t4526;
    let t18658 = t524 * t4539;
    let t19531 = t1559 * t123;
    (t18372, t18535, t18540, t18651, t18658, t19531)
}
