//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1184/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1184(t30988: f64, t31828: f64, t191: f64, t192: f64, t7412: f64, t2020: f64, t6997: f64, t8690: f64, t7000: f64, t6535: f64, t7266: f64, t113: f64, t1869: f64, t30993: f64, t30995: f64, t31034: f64, t31038: f64, t31039: f64, t31041: f64, t7408: f64, t8329: f64) -> (f64, f64, f64) {
    let t31829 = t31828 + t30988;
    let t31832 = t7412 * t191 * t192;
    let t31833 = t31832 * t2020;
    let t31834 = t8690 * t6997;
    let t31835 = t8690 * t7000;
    let t31838 = t7266 * t6535;
    let t31840 = -t113 * t31829 - t1869 * t7408 - t30993 - t30995 - t31034 - t31038 + 3.0_f64 * t31039 - t31041 + t31833 + t31834 - t31835 - 2.0_f64 * t31838 - t8329;
    (t31829, t31832, t31840)
}
