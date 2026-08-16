//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 861/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk861(t44712: f64, t7290: f64, t1841: f64, t7289: f64, t2558: f64, t36390: f64, t9647: f64, t123: f64, t36610: f64, t2563: f64, t35623: f64, t5539: f64) -> (f64, f64, f64, f64, f64) {
    let t44713 = t7290 * t44712;
    let t44716 = 0.17090058289204942852e-2_f64 * t1841 * t7289 * t44713;
    let t44718 = t9647 * t36390 * t2558;
    let t44719 = 0.32043859292259267849e-3_f64 * t44718;
    let t44720 = t36610 * t123;
    let t44722 = t9647 * t44720 * t2563;
    let t44723 = 0.96131577876777803547e-3_f64 * t44722;
    let t44725 = t9647 * t5539 * t35623;
    (t44713, t44716, t44719, t44723, t44725)
}
