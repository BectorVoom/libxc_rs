//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1020/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1020<F: Float>(t35827: F, t30318: F, t537: F, t1165: F, t2068: F, t34681: F, t8600: F, t7433: F, t8908: F, t8912: F, t1181: F, t35618: F, t599: F, t7337: F, t7346: F, t7347: F, t8480: F) -> (F, F, F, F, F, F, F) {
    let t35828 = 0.14291339372689912324e-3 * t35827;
    let t35829 = t30318 * t537;
    let t35833 = t2068 * t1165 * t8600 * t34681;
    let t35835 = t7433 * t8908;
    let t35836 = 0.25724410870841842184e-2 * t35835;
    let t35837 = t7433 * t8912;
    let t35838 = 0.12862205435420921092e-2 * t35837;
    let t35841 = t7337 * t1181 * t599 * t35618;
    let t35844 = t7346 * t8480 * t7347;
    (t35828, t35829, t35833, t35836, t35838, t35841, t35844)
}
