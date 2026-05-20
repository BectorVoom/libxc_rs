//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3614/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3614<F: Float>(t44348: F, t52011: F, t60927: F, t44919: F, t58027: F, t3390: F, t68372: F, t141: F, t3417: F, t68290: F, t43865: F, t43888: F, t43890: F, t43892: F, t58153: F, t58158: F, t58160: F, t58162: F, t58165: F, t58186: F) -> (F, F, F, F, F, F) {
    let t68507 = t52011 * t44348 * t60927;
    let t68515 = t52011 * t44919 * t60927;
    let t68518 = t52011 * t58027 * t60927;
    let t68521 = t3390 * t68372;
    let t68524 = t141 * t3417 * t68290;
    let t68526 = -F::cast_from(0.49057777777777777779e0_f64) * t58153 + F::cast_from(0.73586666666666666666e-1_f64) * t58158 + F::cast_from(0.36793333333333333333e-1_f64) * t58160 + F::new(0.22076e0) * t58162 + F::cast_from(0.14717333333333333333e0_f64) * t68507 - F::cast_from(0.12264444444444444444e0_f64) * t58165 - F::cast_from(0.8945925925925925926e-1_f64) * t43865 - F::cast_from(0.62621481481481481482e0_f64) * t43888 + F::cast_from(0.13418888888888888889e0_f64) * t43890 + F::cast_from(0.26837777777777777778e0_f64) * t43892 - F::new(0.66228e0) * t68515 + F::new(0.198684e1) * t68518 - F::new(0.44152e0) * t58186 - F::new(0.258925e1) * t68521 - F::new(0.16557e0) * t68524;
    (t68507, t68515, t68518, t68521, t68524, t68526)
}
