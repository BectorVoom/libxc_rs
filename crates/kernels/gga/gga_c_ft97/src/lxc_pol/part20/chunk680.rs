//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 680/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk680<F: Float>(t2726: F, t3780: F, t1701: F, t10363: F, t1208: F, t1196: F, t2724: F, t1200: F, t14728: F) -> (F, F, F, F) {
    let t14730 = t3780 * t2726;
    let t14731 = t1701 * t14730;
    let t14734 = t10363 * t1208;
    let t14738 = t2724 * t1196;
    let t14739 = t14738 * t2726;
    let t14742 = t1200 * t14728;
    (t14731, t14734, t14739, t14742)
}
