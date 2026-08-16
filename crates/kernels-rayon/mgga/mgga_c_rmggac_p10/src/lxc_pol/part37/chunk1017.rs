//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1017/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1017(t76031: f64, t76033: f64, t1356: f64, t77831: f64, t11905: f64, t3188: f64, t1971: f64, t2144: f64, t495: f64, t7230: f64, t9540: f64, t15530: f64, t4965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78439 = 0.2553875993597870364e-4_f64 * t76031;
    let t78440 = 0.3830813990396805546e-4_f64 * t76033;
    let t78444 = 0.39914139006212695214e-1_f64 * t1356 * t77831;
    let t78445 = t11905 * t3188;
    let t78446 = 0.14967802127329760705e-1_f64 * t78445;
    let t78450 = t7230 * t1971 * t2144 * t9540 * t495;
    let t78451 = 0.15961724959986689774e-4_f64 * t78450;
    let t78454 = 0.39914139006212695214e-1_f64 * t4965 * t15530;
    (t78439, t78440, t78444, t78446, t78451, t78454)
}
