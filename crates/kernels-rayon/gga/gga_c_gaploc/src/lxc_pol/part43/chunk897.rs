//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 897/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk897(t1897: f64, t2580: f64, t28236: f64, t2958: f64, t1022: f64, t6058: f64, t2508: f64, t28668: f64, t5241: f64, t43107: f64, t7290: f64, t1841: f64, t7289: f64) -> (f64, f64, f64, f64) {
    let t43189 = 0.15381052460284448567e-1_f64 * t1897 * t2580 * t2958 * t28236;
    let t43191 = t6058 * t1022;
    let t43195 = 0.46143157380853345701e0_f64 * t2508 * t43191 * t5241 * t28668;
    let t43199 = t7290 * t43107;
    let t43202 = 0.17090058289204942852e-2_f64 * t1841 * t7289 * t43199;
    (t43189, t43195, t43199, t43202)
}
