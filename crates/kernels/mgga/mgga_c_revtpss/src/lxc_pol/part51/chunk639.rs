//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 639/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk639<F: Float>(t2247: F, t6957: F, t644: F, t84: F, t77: F, t603: F, t607: F, t43: F, t48: F, t624: F, t49: F, t606: F, t613: F) -> (F, F, F, F, F, F) {
    let t6958 = t2247 * t6957;
    let t6959 = t84 * t644;
    let t6960 = t77 * t6959;
    let t6963 = t603 * t607;
    let t6968 = t43 * t48;
    let t6971 = F::new(8.0) / F::new(3.0) * t624;
    let t6972 = -F::new(8.0) / F::new(3.0) * t613 * t49 + F::new(5.0) / F::new(6.0) * t6968 * t606 + t6971;
    (t6958, t6960, t6963, t6968, t6971, t6972)
}
