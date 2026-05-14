//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 723/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk723<F: Float>(t39023: F, t7494: F, t8537: F, t1587: F, t2084: F, t2134: F, t27: F, t7501: F, t8672: F, t1986: F, t2318: F, t305: F, t321: F, t49: F, t529: F, t36940: F, t36945: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t39024 = 0.18183107769496894486e-1 * t39023;
    let t39025 = t7494 * t8537;
    let t39031 = t2134 * t27 * t2084 * t1587;
    let t39048 = t7501 * t8672;
    let t39103 = t1986 * t305 * t2318 * t321;
    let t39116 = t49 * t529;
    let t39119 = t36945 * t39116 * t68 * t36940;
    (t39024, t39025, t39031, t39048, t39103, t39116, t39119)
}
