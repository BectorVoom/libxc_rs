//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1347/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1347<F: Float>(t10795: F, t747: F, t10301: F, t4349: F, t605: F, t10802: F, t14537: F, t1383: F, t17293: F, t3366: F, t17571: F, t3411: F) -> (F, F, F, F, F) {
    let t34013 = t10795 * t747;
    let t34018 = F::new(12.0) * t4349 * t10301 * t605;
    let t34020 = F::new(12.0) * t14537 * t10802;
    let t34023 = F::new(24.0) * t17293 * t3366 * t1383;
    let t34025 = F::new(0.69017266717057349418e1) * t17571 * t3411;
    (t34013, t34018, t34020, t34023, t34025)
}
