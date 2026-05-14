//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 946/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk946<F: Float>(t1181: F, t23688: F, t599: F, t7346: F, t7433: F, t8966: F, t8970: F, t22040: F, t604: F, t7493: F, t21118: F, t7351: F, t7426: F, t1131: F, t525: F, t2068: F) -> (F, F, F, F, F, F, F) {
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35090 = t7433 * t8966;
    let t35092 = t7433 * t8970;
    let t35096 = t7493 * t1181 * t604 * t22040;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35102 = t525 * t1131;
    let t35105 = t2068 * t1181 * t604 * t35102;
    (t35088, t35090, t35092, t35096, t35100, t35102, t35105)
}
