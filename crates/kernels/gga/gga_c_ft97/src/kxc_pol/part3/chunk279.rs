//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 279/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk279<F: Float>(t1073: F, t637: F, t639: F, t1068: F, t629: F, t631: F, t184: F) -> (F, F, F) {
    let t1075 = t637 * t639 * t1073;
    let t1078 = t629 + t631 * t1068 / F::new(6.0) + t631 * t1075 / F::new(2.0);
    let t1079 = t1078 * t184;
    (t1075, t1078, t1079)
}
