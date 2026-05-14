//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1067/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1067<F: Float>(t10295: F, t17288: F, t17277: F, t3366: F, t2358: F, t27232: F, t10629: F, t5227: F, t10632: F, t161: F, t1841: F, t24884: F, t2576: F, t3444: F, t7211: F, t10743: F, t2549: F) -> (F, F, F, F, F, F, F, F) {
    let t32093 = 12.0 * t17288 * t10295;
    let t32095 = 2.0 * t17277 * t3366;
    let t32099 = 4.0 * t27232 * t2358;
    let t32104 = 0.34180116578409885704e-2 * t5227 * t10629;
    let t32106 = 0.51270174867614828558e-2 * t5227 * t10632;
    let t32110 = 0.51270174867614828558e-2 * t1841 * t24884 * t161 * t2576;
    let t32116 = t7211 * t3444;
    let t32117 = 0.32043859292259267849e-3 * t32116;
    let t32118 = t2549 * t10743;
    (t32093, t32095, t32099, t32104, t32106, t32110, t32117, t32118)
}
