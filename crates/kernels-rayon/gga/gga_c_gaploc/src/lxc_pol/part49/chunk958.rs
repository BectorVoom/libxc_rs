//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 958/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk958(t12856: f64, t17288: f64, t2801: f64, t31428: f64, t1016: f64, t1382: f64, t9588: f64, t2902: f64, t3207: f64, t12862: f64, t4342: f64, t12859: f64, t4349: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42485 = 6.0_f64 * t17288 * t12856;
    let t42487 = 2.0_f64 * t31428 * t2801;
    let t42491 = 2.0_f64 * t1382 * t1016 * t9588;
    let t42494 = 2.0_f64 * t1382 * t2902 * t3207;
    let t42496 = 2.0_f64 * t4342 * t12862;
    let t42498 = t4349 * t12859 * t605;
    (t42485, t42487, t42491, t42494, t42496, t42498)
}
