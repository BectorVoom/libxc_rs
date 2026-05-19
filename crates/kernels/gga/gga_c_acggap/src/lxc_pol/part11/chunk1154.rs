//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1154/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1154<F: Float>(t1488: F, t1980: F, t1982: F, t1983: F, t30318: F, t537: F, t1165: F, t2068: F, t34681: F, t8600: F, t7433: F, t8908: F) -> (F, F, F, F) {
    let t35827 = t1980 * t1982 * t1488 * t1983;
    let t35828 = F::cast_from(0.14291339372689912324e-3_f64) * t35827;
    let t35829 = t30318 * t537;
    let t35833 = t2068 * t1165 * t8600 * t34681;
    let t35835 = t7433 * t8908;
    (t35828, t35829, t35833, t35835)
}
