//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 364/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk364(t1445: f64, t2950: f64, t2958: f64, t701: f64, t1035: f64, t773: f64, t1: f64, t1022: f64) -> (f64, f64, f64, f64) {
    let t3028 = t1445 * t2950;
    let t3031 = t2958 * t701;
    let t3032 = t1445 * t3031;
    let t3035 = t773 * t1035;
    let t3038 = t1022 * t1;
    (t3028, t3032, t3035, t3038)
}
