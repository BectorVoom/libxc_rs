//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 94/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk94<F: Float>(t210: F, t213: F, t216: F, t222: F) -> (F, F, F) {
    let t244 = 0.705945e1 * t213 + 0.1549425e1 * t210 + 0.420775e0 * t216 + 0.1562925e0 * t222;
    let t247 = 1.0 + 0.32163958997385070134e2 / t244;
    let t248 = f64::ln(t247);
    (t244, t247, t248)
}
