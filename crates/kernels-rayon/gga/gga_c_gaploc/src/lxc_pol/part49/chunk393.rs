//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 393/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk393(t2562: f64, t3247: f64, t943: f64, t1890: f64, t3209: f64) -> (f64, f64, f64) {
    let t3248 = t2562 * t3247;
    let t3250 = 0.64087718584518535698e-3_f64 * t943 * t3248;
    let t3251 = t1890 * t3209;
    (t3248, t3250, t3251)
}
