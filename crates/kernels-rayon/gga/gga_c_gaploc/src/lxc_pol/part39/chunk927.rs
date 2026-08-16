//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 927/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk927(t12922: f64, t26935: f64, t10497: f64, t9285: f64, t2877: f64, t40251: f64, t12968: f64, t34471: f64, t34286: f64, t10615: f64, t40186: f64, t12964: f64, t587: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41941 = 0.42900587942220512003e1_f64 * t26935 * t12922;
    let t41942 = t9285 * t10497;
    let t41945 = 0.35750489951850426669e0_f64 * t40251 * t2877;
    let t41947 = t34471 * t12968;
    let t41948 = 0.89376224879626066675e-1_f64 * t41947;
    let t41949 = t34286 * t12968;
    let t41950 = 0.89376224879626066675e-1_f64 * t41949;
    let t41951 = t10615 * t40186;
    let t41952 = 0.89376224879626066675e-1_f64 * t41951;
    let t41954 = t587 * t589 * t12964;
    (t41941, t41942, t41945, t41948, t41950, t41952, t41954)
}
