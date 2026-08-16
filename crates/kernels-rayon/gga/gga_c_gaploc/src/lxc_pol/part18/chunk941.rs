//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 941/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk941(t10241: f64, t426: f64, t535: f64, t2268: f64, t2304: f64, t8195: f64, t6767: f64, t7937: f64, t7980: f64, t883: f64, t2325: f64, t882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10242 = t10241 * t426;
    let t10243 = t535 * t10242;
    let t10245 = 0.28455006635676149599e-1_f64 * t2268 * t10243;
    let t10246 = t2304 * t8195;
    let t10248 = 0.19918504644973304719e0_f64 * t2268 * t10246;
    let t10249 = t7937 * t6767;
    let t10251 = 0.34146007962811379518e0_f64 * t2268 * t10249;
    let t10252 = t883 * t7980;
    let t10253 = t2325 * t10252;
    let t10254 = t882 * t10253;
    (t10242, t10243, t10245, t10246, t10248, t10249, t10251, t10253, t10254)
}
