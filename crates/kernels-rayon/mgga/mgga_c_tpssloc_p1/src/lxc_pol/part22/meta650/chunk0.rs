//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2190/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2190(t16398: f64, t19890: f64, t12283: f64, t19972: f64, t16046: f64, t1814: f64, t12250: f64, t5286: f64, t1372: f64, t6414: f64, t1338: f64, t20009: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57450 = t16398 * t19890;
    let t57457 = t12283 * t19972;
    let t57530 = t1814 * t16046;
    let t57568 = t12250 * t5286;
    let t57618 = t1372 * t6414;
    let t57659 = t1338 * t20009;
    (t57450, t57457, t57530, t57568, t57618, t57659)
}
