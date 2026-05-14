//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 786/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk786<F: Float>(t290: F, t9595: F, t1664: F, t2231: F, t42201: F, t42204: F, t42206: F, t42217: F, t942: F, t9639: F, t42238: F, t42242: F, t42246: F, t42258: F, t9640: F, t9642: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t44405 = t290 * t9595;
    let t44410 = t1664 * t2231;
    let t44423 = 0.1454648621559751559e0 * t42201;
    let t44424 = 0.35754263910370185096e-3 * t42204;
    let t44425 = 0.23836175940246790064e-3 * t42206;
    let t44428 = 0.11918087970123395032e-3 * t42217;
    let t44431 = 0.4726e1 * t942 * t9639;
    let t44444 = 0.1440846329149835838e-2 * t42238;
    let t44445 = 0.1440846329149835838e-2 * t42242;
    let t44446 = 0.1440846329149835838e-2 * t42246;
    let t44450 = 0.39726959900411316772e-4 * t42258;
    let t44466 = 0.4726e1 * t9640;
    let t44467 = 0.39914139006212695214e-1 * t9642;
    (t44405, t44410, t44423, t44424, t44425, t44428, t44431, t44444, t44445, t44446, t44450, t44466, t44467)
}
