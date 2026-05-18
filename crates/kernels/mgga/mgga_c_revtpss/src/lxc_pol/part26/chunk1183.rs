//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1183/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1183<F: Float>(t25299: F, t95793: F, t25431: F, t95785: F, t95789: F, t26555: F, t40270: F, t25305: F, t25410: F, t7419: F, t93240: F, t26519: F, t93160: F) -> (F, F, F, F, F, F, F) {
    let t95794 = t25299 * t95793;
    let t95796 = t25431 * t95785;
    let t95798 = t25431 * t95789;
    let t95807 = F::new(0.96373646535613327356e-3) * t40270 * t26555;
    let t95808 = t25305 * t95793;
    let t95811 = t93240 * t25410 * t7419;
    let t95813 = t93160 * t26519;
    (t95794, t95796, t95798, t95807, t95808, t95811, t95813)
}
