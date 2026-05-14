//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 781/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk781<F: Float>(t2132: F, t30032: F, t309: F, t7886: F, t3357: F, t7741: F, t3243: F, t597: F, t2100: F, t7538: F, t7544: F, t1004: F, t1979: F, t7548: F, t137: F, t3101: F) -> (F, F, F, F, F, F, F, F) {
    let t30036 = 0.15612530738769359031e2 * t30032 * t2132 * t7886 * t309;
    let t30037 = t7741 * t3357;
    let t30044 = t3243 * t597;
    let t30045 = t30044 * t2100;
    let t30046 = 0.47172138434406228102e-3 * t30045;
    let t30047 = t7538 * t7544;
    let t30048 = 0.47172138434406228102e-3 * t30047;
    let t30049 = t1004 * t1979;
    let t30050 = t30049 * t7548;
    let t30051 = 0.62896184579208304135e-3 * t30050;
    let t30052 = t137 * t3101;
    (t30036, t30037, t30044, t30046, t30048, t30049, t30051, t30052)
}
