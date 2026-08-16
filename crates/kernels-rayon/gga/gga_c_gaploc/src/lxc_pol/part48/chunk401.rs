//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 401/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk401(t3487: f64, t883: f64, t969: f64, t825: f64, t1445: f64, t3447: f64, t833: f64, t3431: f64, t808: f64, t568: f64, t3309: f64, t2685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3488 = t3487 * t883;
    let t3489 = t969 * t3488;
    let t3490 = t825 * t3489;
    let t3491 = 0.19171462976960374838e0_f64 * t3490;
    let t3492 = t1445 * t3447;
    let t3494 = 0.11502877786176224903e2_f64 * t833 * t3492;
    let t3495 = t808 * t3431;
    let t3496 = t568 * t3495;
    let t3499 = 0.15976219147466979032e-1_f64 * t3309;
    let t3500 = t2685 * t3488;
    (t3488, t3489, t3490, t3491, t3492, t3494, t3495, t3496, t3499, t3500)
}
