//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 941/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk941(t72138: f64, t77050: f64, t74498: f64, t74501: f64, t74503: f64, t15523: f64, t2191: f64, t1986: f64, t675: f64, t9566: f64, t68660: f64, t68686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77051 = t72138 * t77050;
    let t77052 = 0.20455996240684006297e-1_f64 * t77051;
    let t77054 = 0.1276937996798935182e-4_f64 * t74498;
    let t77055 = 0.3830813990396805546e-4_f64 * t74501;
    let t77056 = 0.1276937996798935182e-4_f64 * t74503;
    let t77057 = t2191 * t15523;
    let t77058 = 0.42564599893297839398e-5_f64 * t77057;
    let t77060 = t675 * t1986 * t9566;
    let t77061 = 0.42564599893297839398e-5_f64 * t77060;
    let t77062 = 0.638468998399467591e-4_f64 * t68660;
    let t77069 = 0.36366215538993788974e-1_f64 * t68686;
    (t77052, t77054, t77055, t77056, t77058, t77061, t77062, t77069)
}
