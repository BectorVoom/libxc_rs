//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 956/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk956<F: Float>(t1165: F, t30856: F, t35324: F, t604: F, t33751: F, t7413: F, t1181: F, t599: F, t2297: F, t3176: F, t13287: F, t31195: F, t1980: F, t34487: F, t7476: F, t2118: F, t5082: F) -> (F, F, F, F, F, F, F) {
    let t35327 = t30856 * t1165 * t604 * t35324;
    let t35331 = t7413 * t1165 * t604 * t33751;
    let t35335 = t30856 * t1181 * t599 * t35324;
    let t35340 = t2297 * t3176;
    let t35342 = t31195 * t13287 * t35340;
    let t35348 = t1980 * t7476 * t34487;
    let t35350 = t2118 * t5082;
    (t35327, t35331, t35335, t35340, t35342, t35348, t35350)
}
