//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1100/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1100<F: Float>(t2068: F, t4680: F, t8911: F, t1181: F, t23688: F, t599: F, t7346: F, t7433: F, t8966: F, t8970: F, t22040: F, t604: F, t7493: F) -> (F, F, F, F, F) {
    let t35084 = t2068 * t4680 * t8911;
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35089 = F::cast_from(0.21437009059034868486e-3_f64) * t35088;
    let t35090 = t7433 * t8966;
    let t35092 = t7433 * t8970;
    let t35093 = F::cast_from(0.18868855373762491241e-2_f64) * t35092;
    let t35096 = t7493 * t1181 * t604 * t22040;
    (t35084, t35089, t35090, t35093, t35096)
}
