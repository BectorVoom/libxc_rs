//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 482/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk482(t1: f64, t8025: f64, t544: f64, t188: f64, t7937: f64, t7887: f64, t1415: f64, t2967: f64, t747: f64, t2925: f64, t835: f64, t2936: f64, t769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8330 = t8025 * t1;
    let t8331 = t544 * t8330;
    let t8352 = t188 * t7937;
    let t8410 = t7887 * t1;
    let t8411 = t1415 * t8410;
    let t8440 = t2967 * t747;
    let t8469 = t835 * t2925;
    let t8478 = t769 * t2936;
    (t8331, t8352, t8410, t8411, t8440, t8469, t8478)
}
