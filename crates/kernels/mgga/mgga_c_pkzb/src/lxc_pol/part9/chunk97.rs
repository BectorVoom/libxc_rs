//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 97/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk97<F: Float>(t210: F, t213: F, t216: F, t222: F) -> (F, F, F) {
    let t257 = 0.51785e1 * t213 + 0.905775e0 * t210 + 0.1100325e0 * t216 + 0.1241775e0 * t222;
    let t260 = 1.0 + 0.29608749977793437516e2 / t257;
    let t261 = f64::ln(t260);
    (t257, t260, t261)
}
