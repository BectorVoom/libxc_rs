//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 592/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk592(t10438: f64, t2464: f64, t587: f64, t1407: f64, t3396: f64, t10430: f64, t912: f64, t2293: f64, t2854: f64, t1445: f64, t1562: f64, t3354: f64, t4673: f64) -> (f64, f64, f64, f64, f64) {
    let t10439 = t2464 * t10438;
    let t10440 = t587 * t10439;
    let t10441 = 0.42603251059911944084e-1_f64 * t10440;
    let t10442 = t1407 * t3396;
    let t10443 = 0.19171462976960374838e0_f64 * t10442;
    let t10444 = t912 * t10430;
    let t10445 = t587 * t10444;
    let t10446 = 0.19171462976960374838e0_f64 * t10445;
    let t10447 = t2854 * t2293;
    let t10448 = t1445 * t10447;
    let t10450 = 0.69017266717057349418e1_f64 * t1562 * t10448;
    let t10455 = t4673 * t3354;
    (t10441, t10443, t10446, t10450, t10455)
}
