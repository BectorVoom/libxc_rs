//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1007/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1007(t2842: f64, t5309: f64, t684: f64, t2881: f64, t15191: f64, t4256: f64, t1091: f64, t4176: f64, t10703: f64, t4311: f64, t835: f64, t1255: f64, t3746: f64) -> (f64, f64, f64, f64, f64) {
    let t19585 = t2842 * t5309;
    let t19586 = t19585 * t684;
    let t19587 = t2881 * t19586;
    let t19590 = t15191 * t4256;
    let t19593 = t1091 * t4176;
    let t19594 = t10703 * t19593;
    let t19598 = t835 * t4311 * t1091;
    let t19602 = t835 * t1255 * t3746;
    (t19587, t19590, t19594, t19598, t19602)
}
