//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1080/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1080(t13985: f64, t4593: f64, t4582: f64, t3132: f64, t3069: f64, t4669: f64, t10231: f64, t4338: f64, t973: f64, t13542: f64, t977: f64, t10388: f64, t10424: f64, t10480: f64, t10876: f64, t10898: f64, t10949: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t13977: f64, t13982: f64, t1618: f64, t3073: f64, t3109: f64, t3130: f64, t4596: f64, t4652: f64) -> f64 {
    let t13986 = t4593 * t13985;
    let t13987 = t4582 * t13986;
    let t13990 = t4593 * t3132;
    let t13991 = t4582 * t13990;
    let t13995 = t4669 * t3069;
    let t13998 = t10231 * t4338;
    let t14000 = t973 * t13998 / 324.0_f64;
    let t14001 = t977 * t13542;
    let t14004 = -t10898 * t1618 / 288.0_f64 - t3109 * t4652 / 288.0_f64 + t13959 + t13963 - t13966 / 13824.0_f64 + 11.0_f64 / 324.0_f64 * t10388 - t13972 + t10949 * t4596 / 768.0_f64 + t3130 * t13977 / 768.0_f64 + t3130 * t13982 / 1536.0_f64 + t10480 * t13987 / 512.0_f64 - t10876 * t13991 / 512.0_f64 + t10424 / 3456.0_f64 + t13995 * t3073 / 2304.0_f64 + t14000 - t973 * t14001 / 72.0_f64;
    t14004
}
