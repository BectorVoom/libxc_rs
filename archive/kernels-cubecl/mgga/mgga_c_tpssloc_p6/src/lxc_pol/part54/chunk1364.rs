//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1364/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1364<F: Float>(t27188: F, t6525: F, t1874: F, t92090: F, t33603: F, t6876: F, t31297: F, t7685: F, t191: F, t192: F, t27215: F, t2020: F) -> (F, F, F, F, F) {
    let t121199 = F::cast_from(2.0_f64) * t27188 * t6525;
    let t121201 = F::cast_from(2.0_f64) * t92090 * t1874;
    let t121203 = F::cast_from(3.0_f64) * t6876 * t33603;
    let t121204 = t7685 * t31297;
    let t121210 = t27215 * t191 * t192;
    let t121211 = t121210 * t2020;
    (t121199, t121201, t121203, t121204, t121211)
}
