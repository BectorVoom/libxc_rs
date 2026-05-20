//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1707/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1707<F: Float>(t25895: F, t26234: F, t3920: F, t7496: F, t1398: F, t543: F, t7506: F, t7301: F, t2097: F, t4056: F, t2098: F, t2453: F) -> (F, F, F, F, F) {
    let t26235 = t25895 * t26234;
    let t26238 = F::cast_from(0.13009920719177044025e-1_f64) * t7496 * t3920;
    let t26240 = t7506 * t1398 * t543;
    let t26241 = t7301 * t26240;
    let t26246 = t7301 * t2097 * t4056 * t543;
    let t26249 = t2453 * t2098;
    (t26235, t26238, t26241, t26246, t26249)
}
