//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2043/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2043<F: Float>(t136: F, t2457: F, t7929: F, t25944: F, t2470: F, t27887: F, t7284: F, t1955: F, t27836: F, t4075: F, t26072: F, t27888: F) -> (F, F, F, F, F, F) {
    let t97922 = t7929 * t136 * t2457;
    let t97923 = t25944 * t97922;
    let t97925 = t27887 * t2470;
    let t97926 = t7284 * t97925;
    let t97933 = t1955 * t27836 * t4075;
    let t97943 = F::cast_from(0.14456046980341999104e-1_f64) * t26072 * t27888;
    (t97922, t97923, t97925, t97926, t97933, t97943)
}
