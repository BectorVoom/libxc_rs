//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 408/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk408(t1445: f64, t3410: f64, t1562: f64, t1024: f64, t954: f64, t2508: f64, t3216: f64, t3226: f64, t3218: f64, t3223: f64, t471: f64, t1035: f64, t2558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3411 = t1445 * t3410;
    let t3413 = 0.69017266717057349418e1_f64 * t1562 * t3411;
    let t3420 = t954 * t1024;
    let t3422 = 0.76905262301422242837e-2_f64 * t2508 * t3420;
    let t3423 = 3.0_f64 / 128.0_f64 * t3216;
    let t3426 = t3226 / 128.0_f64;
    let t3427 = t3423 - 9.0_f64 / 4096.0_f64 * t3218 + 3.0_f64 / 4096.0_f64 * t3223 - t3426;
    let t3428 = t3427 * t471;
    let t3437 = t1035 * t2558;
    (t3411, t3413, t3420, t3422, t3423, t3426, t3427, t3428, t3437)
}
