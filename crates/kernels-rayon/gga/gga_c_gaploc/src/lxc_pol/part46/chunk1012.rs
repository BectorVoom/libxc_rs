//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1012/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1012(t44088: f64, t3039: f64, t5774: f64, t3277: f64, t13009: f64, t5782: f64, t1457: f64, t43240: f64, t6060: f64, t13158: f64, t15766: f64, t41425: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44089 = 0.15976219147466979032e-1_f64 * t44088;
    let t44090 = t3039 * t5774;
    let t44092 = 0.16683561977530199113e1_f64 * t3277 * t44090;
    let t44093 = t5782 * t13009;
    let t44097 = 0.21450293971110256001e1_f64 * t6060 * t1457 * t43240;
    let t44099 = 0.21450293971110256001e1_f64 * t15766 * t13158;
    let t44106 = 0.1022478025437886658e1_f64 * t41425;
    (t44089, t44092, t44093, t44097, t44099, t44106)
}
