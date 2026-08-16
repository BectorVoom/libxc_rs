//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 893/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk893(t1016: f64, t1382: f64, t9588: f64, t2902: f64, t3207: f64, t12862: f64, t4342: f64, t12859: f64, t4349: f64, t605: f64, t10301: f64, t6556: f64) -> (f64, f64, f64, f64, f64) {
    let t42491 = 2.0_f64 * t1382 * t1016 * t9588;
    let t42494 = 2.0_f64 * t1382 * t2902 * t3207;
    let t42496 = 2.0_f64 * t4342 * t12862;
    let t42498 = t4349 * t12859 * t605;
    let t42499 = 12.0_f64 * t42498;
    let t42501 = 4.0_f64 * t6556 * t10301;
    (t42491, t42494, t42496, t42499, t42501)
}
