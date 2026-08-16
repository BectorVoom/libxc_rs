//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2105/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2105(t19440: f64, t71: f64, t33: f64, t55880: f64, t5441: f64, t645: f64, t72: f64, t5389: f64, t641: f64, t12568: f64, t1410: f64, t1860: f64, t1863: f64, t1865: f64, t22544: f64, t26084: f64, t26090: f64, t27950: f64, t27953: f64, t27956: f64, t27957: f64, t27961: f64, t6490: f64, t6495: f64, t6505: f64, t83741: f64, t83827: f64) -> f64 {
    let t96379 = t71 * t19440;
    let t96383 = t55880 * t33;
    let t96393 = t72 * t5441 * t645;
    let t96403 = t72 * t641 * t5389;
    let t96406 = t12568 * t1410;
    let t96409 = -t1860 * t6505 * t27956 / 6.0_f64 - t1860 * t1863 * t96379 / 6.0_f64 - t96383 * t1865 / 6.0_f64 + t6495 * t27950 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t26084 * t26090 + 2.0_f64 / 3.0_f64 * t6495 * t27953 + 5.0_f64 / 6.0_f64 * t6490 * t96393 + t6495 * t27957 / 3.0_f64 - 5.0_f64 * t83827 * t27961 - 5.0_f64 * t83741 * t27961 - 5.0_f64 * t22544 * t96403 + 2.0_f64 / 3.0_f64 * t96406 * t1865;
    t96409
}
