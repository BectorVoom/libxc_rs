//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1027/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1027<F: Float>(t1393: F, t1459: F, t1849: F, t24932: F, t26166: F, t26170: F, t26178: F, t26181: F, t26183: F, t26505: F, t27879: F, t27888: F, t27903: F, t4037: F, t4073: F, t4077: F, t574: F, t652: F, t7266: F, t7412: F, t8107: F) -> F {
    let t27905 = t1393 * t8107 - F::cast_from(2.0_f64) * t1459 * t24932 - F::cast_from(2.0_f64) * t1459 * t27888 + t1849 * t7412 - F::cast_from(2.0_f64) * t27879 * t652 + t27903 * t574 - F::cast_from(2.0_f64) * t4037 * t7266 - F::cast_from(2.0_f64) * t4073 * t7266 - F::cast_from(2.0_f64) * t4077 * t7266 + t26166 + t26170 - t26178 - t26181 - t26183 + t26505;
    t27905
}
