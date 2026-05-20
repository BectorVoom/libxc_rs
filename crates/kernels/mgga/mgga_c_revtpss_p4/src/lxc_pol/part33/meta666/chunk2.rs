//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2181/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2181<F: Float>(t2018: F, t22129: F, t807: F, t22262: F, t25986: F, t2661: F, t22182: F, t94508: F, t102486: F, t102489: F, t102495: F, t94444: F, t94460: F, t98145: F, t98147: F, t98152: F, t98157: F) -> F {
    let t108554 = t807 * t2018 * t22129;
    let t108559 = t2661 * t25986 * t22262;
    let t108562 = t94508 * t22182;
    let t108564 = -F::cast_from(0.28582678745379824648e-3_f64) * t108554 + F::cast_from(0.10841600599314203355e-2_f64) * t94444 - t102486 + t98145 + t98147 + t102489 - F::cast_from(0.57165357490759649295e-3_f64) * t98152 - t98157 + t102495 - F::cast_from(0.11433071498151929859e-3_f64) * t108559 - F::cast_from(0.11337795902333997111e-1_f64) * t94460 + F::cast_from(0.50820002809285328225e-4_f64) * t108562;
    t108564
}
