//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1866/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1866<F: Float>(t1096: F, t7817: F, t7160: F, t7821: F, t988: F, t7145: F, t1035: F, t7810: F, t1043: F, t1089: F, t1982: F, t27418: F) -> (F, F, F, F, F) {
    let t27594 = t7817 * t1096;
    let t27595 = t7160 * t27594;
    let t27598 = t7821 * t988;
    let t27599 = t7145 * t27598;
    let t27604 = t1035 * t7810;
    let t27606 = t27604 * t1043 * t1089;
    let t27609 = t1982 * t27418;
    (t27595, t27599, t27604, t27606, t27609)
}
