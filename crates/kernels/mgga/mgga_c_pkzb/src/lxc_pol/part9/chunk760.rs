//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 760/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk760<F: Float>(t160: F, t162: F, t1742: F, t1747: F, t1750: F, t2631: F, t5348: F, t5357: F, t5361: F, t5364: F, t594: F, t597: F) -> F {
    let t5367 = F::cast_from(60.0_f64) * t160 * t5357 + F::cast_from(3.0_f64) * t160 * t5364 - t162 * t5348 + F::cast_from(9.0_f64) * t1742 * t597 - F::cast_from(36.0_f64) * t1747 * t594 + F::cast_from(9.0_f64) * t1750 * t594 - F::cast_from(36.0_f64) * t2631 * t5361;
    t5367
}
