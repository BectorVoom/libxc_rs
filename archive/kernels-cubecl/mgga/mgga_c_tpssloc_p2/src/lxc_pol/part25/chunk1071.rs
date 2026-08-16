//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1071/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1071<F: Float>(t1878: F, t23033: F, t253: F, t254: F, t234: F, t6604: F, t10143: F, t25: F, t28: F, t870: F, t1982: F, t8944: F) -> (F, F, F, F, F, F, F) {
    let t25154 = t1878 * t23033;
    let t25168 = t253 * t254;
    let t25248 = t6604 * t234;
    let t25373 = t10143 * t25;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26161 = t1982 * t8944;
    (t25154, t25168, t25248, t25373, t25891, t25927, t26161)
}
