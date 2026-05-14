//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 853/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk853<F: Float>(t6141: F, t935: F, t915: F, t2926: F, t6109: F, t2924: F, t2930: F, t4571: F, t6094: F, t6098: F, t6102: F, t1621: F, t954: F, t2950: F, t2957: F, t4620: F, t6114: F, t6121: F, t6127: F, t6129: F, t6133: F, t6136: F, t6139: F) -> (F, F, F, F, F, F, F, F) {
    let t6142 = t6141 * t935;
    let t6144 = 1.0 * t915 * t6142;
    let t6145 = t6109 * t2926;
    let t6147 = 0.16081979498692535067e2 * t2924 * t6145;
    let t6152 = t2930 + 0.11415555555555555555e-1 * t4571 - 0.11415555555555555555e-1 * t6094 + 0.34246666666666666666e-1 * t6098 - 0.17123333333333333333e-1 * t6102;
    let t6157 = t1621 * t1621;
    let t6158 = t6157 * t954;
    let t6173 = -0.17648625e1 * t6114 + 0.3529725e1 * t6121 + t2950 + 0.34431666666666666666e0 * t4571 - 0.34431666666666666667e0 * t6094 + 0.103295e1 * t6098 - 0.516475e0 * t6102 + 0.31558125e0 * t6127 + 0.6311625e0 * t6129 + t2957 + 0.13892666666666666667e0 * t4620 - 0.34731666666666666667e-1 * t6133 + 0.20839e0 * t6136 - 0.104195e0 * t6139;
    (t6142, t6144, t6145, t6147, t6152, t6157, t6158, t6173)
}
