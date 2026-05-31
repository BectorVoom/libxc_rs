//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1206/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1206<F: Float>(t2358: F, t27232: F, t10281: F, t501: F, t605: F, t10629: F, t5227: F, t10632: F, t161: F, t1841: F, t24884: F, t2576: F) -> (F, F, F, F, F) {
    let t32099 = F::cast_from(4.0_f64) * t27232 * t2358;
    let t32100 = t10281 * t501;
    let t32102 = F::cast_from(2.0_f64) * t32100 * t605;
    let t32104 = F::cast_from(0.34180116578409885704e-2_f64) * t5227 * t10629;
    let t32106 = F::cast_from(0.51270174867614828558e-2_f64) * t5227 * t10632;
    let t32110 = F::cast_from(0.51270174867614828558e-2_f64) * t1841 * t24884 * t161 * t2576;
    (t32099, t32102, t32104, t32106, t32110)
}
