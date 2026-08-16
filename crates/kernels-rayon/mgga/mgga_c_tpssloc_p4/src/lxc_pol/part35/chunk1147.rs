//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1147/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1147(t24847: f64, t24848: f64, t1089: f64, t491: f64, t7327: f64, t11947: f64, t2157: f64, t111: f64, t2169: f64, t192: f64, t531: f64, t1982: f64) -> (f64, f64, f64, f64, f64) {
    let t24849 = t24847 * t24848;
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24909 = t2157 * t11947;
    let t24972 = t2169 * t111;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    (t24849, t24851, t24909, t24972, t24995)
}
