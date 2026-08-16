//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 872/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk872(t44888: f64, t7290: f64, t43093: f64, t43100: f64, t1897: f64, t35583: f64, t954: f64, t2508: f64, t44712: f64, t688: f64, t779: f64, t1023: f64, t44878: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44889 = t7290 * t44888;
    let t44895 = 0.1281754371690370714e-2_f64 * t43093;
    let t44898 = 0.1281754371690370714e-2_f64 * t43100;
    let t44901 = 0.76905262301422242837e-2_f64 * t1897 * t954 * t35583;
    let t44905 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t44712 * t688;
    let t44906 = t1023 * t44878;
    (t44889, t44895, t44898, t44901, t44905, t44906)
}
