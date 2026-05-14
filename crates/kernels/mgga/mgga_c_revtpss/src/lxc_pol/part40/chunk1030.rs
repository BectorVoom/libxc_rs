//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1030/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1030<F: Float>(t3515: F, t3523: F, t1156: F, t3451: F, t12295: F, t12351: F, t1178: F, t3519: F, t439: F, t3522: F, t447: F, t300: F, t3488: F, t3800: F, t498: F, t1204: F, t1269: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12500 = t3515 * t3523;
    let t12511 = t1156 * t3451;
    let t12542 = 0.93932222222222222223e0 * t12295;
    let t12543 = 0.36793333333333333333e0 * t12351;
    let t12552 = 1.0 / t3519 / t1178;
    let t12553 = t439 * t12552;
    let t12555 = 1.0 / t3522 / t447;
    let t12571 = t300 * t3488;
    let t12587 = 1.0 / t3800 / t498;
    let t12603 = t1204 * t1269;
    (t12500, t12511, t12542, t12543, t12552, t12553, t12555, t12571, t12587, t12603)
}
