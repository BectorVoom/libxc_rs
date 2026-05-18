//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 963/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk963<F: Float>(t12032: F, t2902: F, t14295: F, t4342: F, t12148: F, t2798: F, t1016: F, t39340: F, t1382: F, t4349: F, t605: F, t1022: F, t3720: F) -> (F, F, F, F, F, F, F) {
    let t49970 = F::new(2.0) * t12032 * t2902;
    let t49972 = F::new(4.0) * t4342 * t14295;
    let t49974 = F::new(2.0) * t2798 * t12148;
    let t49977 = F::new(2.0) * t39340 * t1016;
    let t49980 = F::new(4.0) * t1382 * t1016 * t12148;
    let t49983 = F::new(12.0) * t4349 * t14295 * t605;
    let t49989 = t1022 * t3720;
    (t49970, t49972, t49974, t49977, t49980, t49983, t49989)
}
