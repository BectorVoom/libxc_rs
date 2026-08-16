//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1013/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1013(t11707: f64, t3609: f64, t3623: f64, t3620: f64, t5079: f64, t10471: f64, t1209: f64, t11712: f64, t475: f64, t6739: f64, t11882: f64, t11616: f64, t11621: f64, t11625: f64, t11640: f64, t11869: f64, t11872: f64, t11877: f64, t11881: f64, t11884: f64, t11888: f64, t11890: f64, t11893: f64, t11897: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t3565: f64, t3604: f64, t3610: f64, t3613: f64, t3617: f64, t3621: f64, t3624: f64, t3626: f64, t3628: f64, t470: f64, t494: f64) -> (f64, f64, f64, f64, f64) {
    let t11904 = t11707 * t3609;
    let t11907 = t11707 * t3623;
    let t11910 = t3620 * t5079;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11915 = t6739 * t475;
    let t11916 = t11882 * t11915;
    let t11918 = t11616 * t494 + 3.0_f64 * t3604 * t3621 + 3.0_f64 * t1244 * t11621 - 3.0_f64 * t3624 * t11625 + t1244 * t11640 + t470 * t11869 + 6.0_f64 * t3610 * t11872 + 6.0_f64 * t3604 * t3617 + 3.0_f64 * t11877 * t1247 + 6.0_f64 * t11881 * t11884 - 6.0_f64 * t11888 * t11890 + 6.0_f64 * t3610 * t11893 + 3.0_f64 * t1244 * t11897 + 3.0_f64 * t3565 * t1249 + 3.0_f64 * t1201 * t3628 + 6.0_f64 * t11904 * t3613 - 3.0_f64 * t11907 * t3626 - 3.0_f64 * t3624 * t11910 + t11914 * t11916;
    (t11904, t11907, t11914, t11915, t11918)
}
