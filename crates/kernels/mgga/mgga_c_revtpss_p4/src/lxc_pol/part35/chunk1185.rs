//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1185/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1185<F: Float>(t1711: F, t5966: F, t6079: F, t23279: F, t27763: F, t6075: F, t23421: F, t33: F, t113096: F, t25759: F, t23148: F, t1583: F, t6416: F) -> (F, F, F, F, F, F, F, F) {
    let t114117 = t1711 * t5966;
    let t114121 = t1711 * t6079;
    let t114128 = t27763 * t23279;
    let t114140 = t1711 * t6075;
    let t114150 = t33 * t23421;
    let t114165 = t25759 * t113096;
    let t114171 = t33 * t23148;
    let t114184 = t6416 * t1583;
    (t114117, t114121, t114128, t114140, t114150, t114165, t114171, t114184)
}
