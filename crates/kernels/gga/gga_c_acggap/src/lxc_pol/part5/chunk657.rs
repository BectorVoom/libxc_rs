//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 657/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk657<F: Float>(t1535: F, t4389: F, t1165: F, t1533: F, t4289: F, t1162: F, t4180: F) -> (F, F, F) {
    let t4391 = F::cast_from(0.40015750243531754508e-2_f64) * t4389 * t1535;
    let t4393 = t1165 * t4289 * t1533;
    let t4396 = t4180 * t1162;
    (t4391, t4393, t4396)
}
