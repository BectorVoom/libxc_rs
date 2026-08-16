//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1115/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1115<F: Float>(t35301: F, t1181: F, t20817: F, t599: F, t7337: F, t7433: F, t8779: F, t4979: F, t7561: F, t4983: F, t7822: F, t21955: F, t30806: F) -> (F, F, F, F, F, F) {
    let t35302 = F::cast_from(0.15724046144802076034e-2_f64) * t35301;
    let t35305 = t7337 * t1181 * t599 * t20817;
    let t35307 = t7433 * t8779;
    let t35308 = F::cast_from(0.25724410870841842184e-2_f64) * t35307;
    let t35309 = t7561 * t4979;
    let t35311 = t7822 * t4983;
    let t35315 = t30806 * t1181 * t599 * t21955;
    (t35302, t35305, t35308, t35309, t35311, t35315)
}
