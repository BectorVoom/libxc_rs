//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2665/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2665<F: Float>(t19676: F, t3127: F, t3172: F, t16158: F, t4834: F, t19791: F, t19781: F, t3091: F, t43131: F, t19939: F, t11262: F, t3161: F, t6311: F) -> (F, F, F, F, F, F) {
    let t65527 = t3127 * t3172 * t19676;
    let t65538 = t4834 * t16158;
    let t65553 = t3127 * t3172 * t19791;
    let t65567 = t3091 * t43131 * t19781;
    let t65570 = t3127 * t3172 * t19939;
    let t65581 = t3161 * t11262 * t6311;
    (t65527, t65538, t65553, t65567, t65570, t65581)
}
