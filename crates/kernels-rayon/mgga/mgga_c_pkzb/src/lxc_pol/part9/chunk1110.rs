//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1110/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1110(t300: f64, t6404: f64, t394: f64, t6406: f64, t2099: f64, t6459: f64, t6463: f64, t2320: f64, t6224: f64, t2255: f64, t2277: f64, t356: f64) -> (f64, f64, f64, f64, f64) {
    let t18661 = t300 * t6404;
    let t18662 = t394 * t6406;
    let t18668 = t6459 * t2099 * t6463;
    let t18679 = t6224 * t2320;
    let t18706 = t356 / t2277 / t2255;
    (t18661, t18662, t18668, t18679, t18706)
}
