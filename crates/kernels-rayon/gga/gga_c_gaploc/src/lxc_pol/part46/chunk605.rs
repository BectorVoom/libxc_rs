//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 605/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk605(t10442: f64, t10430: f64, t912: f64, t587: f64, t2293: f64, t2854: f64, t1445: f64, t1562: f64, t10151: f64, t447: f64, t3354: f64, t4673: f64) -> (f64, f64, f64, f64, f64) {
    let t10443 = 0.19171462976960374838e0_f64 * t10442;
    let t10444 = t912 * t10430;
    let t10445 = t587 * t10444;
    let t10446 = 0.19171462976960374838e0_f64 * t10445;
    let t10447 = t2854 * t2293;
    let t10448 = t1445 * t10447;
    let t10450 = 0.69017266717057349418e1_f64 * t1562 * t10448;
    let t10451 = t10151 * t447;
    let t10452 = t1445 * t10451;
    let t10455 = t4673 * t3354;
    (t10443, t10446, t10450, t10452, t10455)
}
