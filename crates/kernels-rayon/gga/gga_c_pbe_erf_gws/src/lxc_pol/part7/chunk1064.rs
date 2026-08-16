//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1064/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1064(t159: f64, t285: f64, t4562: f64, t532: f64, t545: f64, t5676: f64, t102: f64, t1533: f64, t1544: f64, t497: f64, t5645: f64, t413: f64, t5772: f64, t5773: f64) -> (f64, f64, f64, f64, f64) {
    let t19206 = t532 * t4562 * t159 * t285;
    let t19209 = t5676 * t545 * t285;
    let t19216 = 0.1052289e3_f64 * t102 * t1544 * t1533;
    let t19219 = 0.233842e2_f64 * t102 * t497 * t5645;
    let t19229 = 0.15589466666666666666e2_f64 * t5772 * t5773 * t413;
    (t19206, t19209, t19216, t19219, t19229)
}
