//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1146/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1146<F: Float>(t2018: F, t22129: F, t807: F, t22262: F, t25986: F, t2661: F, t22182: F, t94508: F, t22267: F, t25997: F, t22259: F, t26024: F, t6876: F, t22125: F, t6864: F, t94455: F) -> (F, F, F, F, F, F, F, F) {
    let t108554 = t807 * t2018 * t22129;
    let t108559 = t2661 * t25986 * t22262;
    let t108562 = t94508 * t22182;
    let t108566 = t25997 * t22267;
    let t108570 = t25997 * t22259;
    let t108576 = t26024 * t6876;
    let t108587 = t807 * t2018 * t22125;
    let t108590 = t94455 * t6864;
    (t108554, t108559, t108562, t108566, t108570, t108576, t108587, t108590)
}
