//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 850/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk850(t2877: f64, t40251: f64, t12968: f64, t34471: f64, t34286: f64, t10615: f64, t40186: f64, t12964: f64, t587: f64, t589: f64, t1429: f64, t2365: f64, t2366: f64, t31747: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41945 = 0.35750489951850426669e0_f64 * t40251 * t2877;
    let t41947 = t34471 * t12968;
    let t41948 = 0.89376224879626066675e-1_f64 * t41947;
    let t41949 = t34286 * t12968;
    let t41950 = 0.89376224879626066675e-1_f64 * t41949;
    let t41951 = t10615 * t40186;
    let t41952 = 0.89376224879626066675e-1_f64 * t41951;
    let t41954 = t587 * t589 * t12964;
    let t41958 = t1429 * t2365 * t2366 * t31747;
    (t41945, t41948, t41950, t41952, t41954, t41958)
}
