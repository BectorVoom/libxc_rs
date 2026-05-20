//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1855/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1855<F: Float>(t26519: F, t93160: F, t25372: F, t95536: F, t7398: F, t822: F, t93170: F, t95746: F, t7064: F, t95575: F, t2067: F, t41117: F) -> (F, F, F, F, F, F) {
    let t95813 = t93160 * t26519;
    let t95822 = t25372 * t95536;
    let t95825 = t822 * t7398;
    let t95836 = t93170 * t95746;
    let t95859 = t7064 * t95575;
    let t95862 = F::cast_from(0.81814717454467823679e-4_f64) * t41117 * t2067;
    (t95813, t95822, t95825, t95836, t95859, t95862)
}
