//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3614/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3614(t44348: f64, t52011: f64, t60927: f64, t44919: f64, t58027: f64, t3390: f64, t68372: f64, t141: f64, t3417: f64, t68290: f64, t43865: f64, t43888: f64, t43890: f64, t43892: f64, t58153: f64, t58158: f64, t58160: f64, t58162: f64, t58165: f64, t58186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68507 = t52011 * t44348 * t60927;
    let t68515 = t52011 * t44919 * t60927;
    let t68518 = t52011 * t58027 * t60927;
    let t68521 = t3390 * t68372;
    let t68524 = t141 * t3417 * t68290;
    let t68526 = -0.49057777777777777779e0_f64 * t58153 + 0.73586666666666666666e-1_f64 * t58158 + 0.36793333333333333333e-1_f64 * t58160 + 0.22076e0_f64 * t58162 + 0.14717333333333333333e0_f64 * t68507 - 0.12264444444444444444e0_f64 * t58165 - 0.8945925925925925926e-1_f64 * t43865 - 0.62621481481481481482e0_f64 * t43888 + 0.13418888888888888889e0_f64 * t43890 + 0.26837777777777777778e0_f64 * t43892 - 0.66228e0_f64 * t68515 + 0.198684e1_f64 * t68518 - 0.44152e0_f64 * t58186 - 0.258925e1_f64 * t68521 - 0.16557e0_f64 * t68524;
    (t68507, t68515, t68518, t68521, t68524, t68526)
}
