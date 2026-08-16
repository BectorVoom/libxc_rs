//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1839/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1839<F: Float>(t23967: F, t26090: F, t23993: F, t7435: F, t46104: F, t7025: F, t26055: F, t7032: F, t26063: F, t7432: F, t84241: F, t45844: F) -> (F, F, F, F, F, F, F) {
    let t91904 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t23967 * t26090;
    let t91905 = t7435 * t23993;
    let t91907 = t46104 * t7025;
    let t91913 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t26055 * t7032;
    let t91921 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t23967 * t26063;
    let t91922 = t84241 * t7432;
    let t91954 = t45844 * t7025;
    (t91904, t91905, t91907, t91913, t91921, t91922, t91954)
}
