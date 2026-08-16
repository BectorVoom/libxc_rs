//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1769/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1769<F: Float>(t22822: F, t281: F, t6589: F, t23124: F, t23076: F, t6597: F, t23047: F, t2617: F, t2690: F, t6612: F, t812: F, t831: F) -> (F, F, F, F, F, F) {
    let t81788 = t22822 * t6589 * t281;
    let t81789 = t81788 * t23124;
    let t81792 = t6597 * t23076 * t281;
    let t81803 = t2617 * t23047;
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    (t81788, t81789, t81792, t81803, t81807, t81808)
}
