//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 623/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk623(t11724: f64, t723: f64, t1445: f64, t3621: f64, t590: f64, t3626: f64, t10814: f64, t10824: f64, t10829: f64, t10835: f64, t10838: f64, t10841: f64, t10844: f64, t11721: f64, t1966: f64, t1991: f64, t2103: f64, t813: f64) -> f64 {
    let t11725 = t11724 * t723;
    let t11726 = t1445 * t11725;
    let t11730 = t3621 * t590;
    let t11733 = t3626 * t590;
    let t11742 = 0.71500979903700853338e0_f64 * t2103 * t11721 - 0.92023022289409799224e1_f64 * t813 * t11726 - 0.11916829983950142223e0_f64 * t10814 + 0.1022478025437886658e1_f64 * t1991 * t11730 - 0.25561950635947166451e1_f64 * t1966 * t11733 - 0.76685851907841499353e0_f64 * t10824 - 0.76685851907841499353e0_f64 * t10829 + 0.59584149919750711116e-1_f64 * t10835 - 0.1022478025437886658e1_f64 * t10838 + 0.11916829983950142223e0_f64 * t10841 + 0.1022478025437886658e1_f64 * t10844;
    t11742
}
