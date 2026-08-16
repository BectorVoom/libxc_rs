//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 630/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk630(t3601: f64, t5750: f64, t723: f64, t1445: f64, t11623: f64, t11603: f64, t701: f64, t1: f64, t11656: f64, t787: f64, t11661: f64, t1589: f64, t3626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11832 = t5750 * t3601;
    let t11833 = t11832 * t723;
    let t11834 = t1445 * t11833;
    let t11837 = t1445 * t11623;
    let t11840 = t11603 * t701;
    let t11841 = t1445 * t11840;
    let t11844 = t11656 * t1;
    let t11845 = t787 * t11844;
    let t11848 = t11661 * t1;
    let t11849 = t787 * t11848;
    let t11854 = t1589 * t3626;
    (t11832, t11834, t11837, t11841, t11844, t11845, t11848, t11849, t11854)
}
