//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 882/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk882<F: Float>(t3698: F, t65: F, t5047: F, t1234: F, t8184: F, t5362: F, t7613: F, t1230: F, t1256: F, t8177: F, t2138: F, t5261: F, t8185: F, t1238: F, t1791: F, t26827: F, t26855: F, t26863: F, t29047: F, t484: F, t5320: F) -> (F,) {
    let t29054 = t65 * t3698;
    let t29055 = t29054 * t5047;
    let t29062 = t1234 * t8184;
    let t29065 = t7613 * t5362;
    let t29069 = t1230 * t8184;
    let t29072 = t8177 * t1256;
    let t29074 = t5261 * t2138;
    let t29077 = t8185 * t1256;
    let t29079 = t29047 * t29055 / 216.0 - 0.42874018118069736972e-3 * t26827 * t1791 - 0.42874018118069736972e-3 * t7613 * t5320 + 0.22866142996303859718e-2 * t29062 * t1238 - 0.28582678745379824648e-3 * t29065 - 0.19055119163586549765e-3 * t26855 + 0.28582678745379824648e-3 * t26863 - 0.22866142996303859718e-2 * t29069 * t484 + 0.28582678745379824648e-3 * t29072 + 0.42874018118069736972e-3 * t29074 * t484 - 0.15244095330869239812e-2 * t29077;
    (t29079,)
}
