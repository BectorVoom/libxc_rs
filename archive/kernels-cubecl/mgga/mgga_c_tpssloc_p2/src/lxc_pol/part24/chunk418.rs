//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 418/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk418<F: Float>(t1915: F, t25: F, t1877: F, t337: F, t38: F, t1887: F) -> (F, F, F, F) {
    let t1916 = t1915 * t25;
    let t1918 = t1877 * t1916 / F::cast_from(2.0_f64);
    let t1919 = t38 * t337;
    let t1920 = t1919 * t1887;
    (t1916, t1918, t1919, t1920)
}
