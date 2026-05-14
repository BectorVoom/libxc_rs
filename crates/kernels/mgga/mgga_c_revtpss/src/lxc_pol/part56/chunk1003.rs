//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1003/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1003<F: Float>(t127236: F, t127287: F, t32099: F, t7898: F, t25082: F, t27153: F, t37110: F, t33974: F, t531: F, t2014: F, t7238: F, t32298: F, t33976: F, t7235: F, t28167: F, t37972: F, t5627: F) -> (F, F, F, F, F, F, F) {
    let t127288 = t127236 + t127287;
    let t127302 = 3.0 * t7898 * t32099;
    let t127305 = 6.0 * t25082 * t37110 * t27153;
    let t127310 = t531 * t33974;
    let t127313 = 3.0 * t2014 * t127310 * t7238;
    let t127332 = t7898 * t32298;
    let t127335 = t7235 * t33976;
    let t127340 = 6.0 * t28167 * t37972 * t5627;
    (t127288, t127302, t127305, t127313, t127332, t127335, t127340)
}
