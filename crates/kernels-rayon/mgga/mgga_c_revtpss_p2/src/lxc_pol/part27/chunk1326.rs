//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1326/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1326(t265: f64, t502: f64, t27033: f64, t3801: f64, t12587: f64, t7669: f64, t2155: f64, t44126: f64, t12584: f64, t1298: f64, t1300: f64, t13190: f64, t198: f64, t27037: f64, t27041: f64, t336: f64, t3794: f64, t3798: f64, t5023: f64, t60126: f64, t7673: f64, t94213: f64, t96913: f64, t96964: f64, t97015: f64, t97072: f64, t97323: f64, t97375: f64, t97428: f64, t97480: f64) -> f64 {
    let t503 = t265 < t502;
    let t97487 = t27033 * t3801;
    let t97491 = t7669 * t12587;
    let t97498 = t2155 * t44126;
    let t97508 = piecewise3(t503, t198 * t336 * (t96913 + t96964 + t97015 + t97072 + t97323 + t97375 + t97428 + t97480) * t1300 - 3.0_f64 * t5023 * t97487 * t1298 + 6.0_f64 * t5023 * t97491 * t3798 - 3.0_f64 * t5023 * t27037 * t3794 - 6.0_f64 * t5023 * t97498 * t12584 + 6.0_f64 * t5023 * t27041 * t60126 - t5023 * t7673 * t13190, t94213);
    t97508
}
