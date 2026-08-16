//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1023/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1023(t1403: f64, t1689: f64, t1509: f64, t5685: f64, t1037: f64, t1303: f64, t4048: f64, t6: f64, t1153: f64, t1418: f64, t122: f64, t169: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21115 = t1689 * t1403;
    let t21157 = t5685 * t1509;
    let t21183 = t1037 * t1303;
    let t21204 = t4048 * t6;
    let t21249 = t1418 * t1153;
    let t21281 = t169 * t4048 * t122 * t188;
    (t21115, t21157, t21183, t21204, t21249, t21281)
}
