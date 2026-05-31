//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 95/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk95<F: Float>(t210: F, t213: F, t216: F, t222: F) -> (F, F, F) {
    let t244 = F::cast_from(0.705945e1_f64) * t213 + F::cast_from(0.1549425e1_f64) * t210 + F::cast_from(0.420775e0_f64) * t216 + F::cast_from(0.1562925e0_f64) * t222;
    let t247 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t244;
    let t248 = F::ln(t247);
    (t244, t247, t248)
}
