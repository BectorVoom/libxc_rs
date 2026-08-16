//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3124/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3124(t11922: f64, t16067: f64, t16069: f64, t11200: f64, t380: f64, t16088: f64, t3105: f64, t4797: f64, t15725: f64, t15827: f64, t11921: f64, t16152: f64, t247: f64, t4837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55328 = t16067 * t11922 * t16069;
    let t55330 = t11200 * t380;
    let t55331 = t55330 * t16088;
    let t55356 = t4797 * t3105;
    let t55361 = t15725 * t15827;
    let t55367 = t4837 * t247 * t11921 * t16152;
    (t55328, t55330, t55331, t55356, t55361, t55367)
}
