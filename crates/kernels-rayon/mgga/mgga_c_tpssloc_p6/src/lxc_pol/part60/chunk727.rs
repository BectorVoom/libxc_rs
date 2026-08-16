//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 727/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk727(t7284: f64, t974: f64, t24847: f64, t1089: f64, t491: f64, t7327: f64, t11947: f64, t2157: f64, t111: f64, t2169: f64, t192: f64, t531: f64) -> (f64, f64, f64, f64, f64) {
    let t24848 = t974 * t7284;
    let t24849 = t24847 * t24848;
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24909 = t2157 * t11947;
    let t24972 = t2169 * t111;
    let t24994 = t192 * t531;
    (t24849, t24851, t24909, t24972, t24994)
}
