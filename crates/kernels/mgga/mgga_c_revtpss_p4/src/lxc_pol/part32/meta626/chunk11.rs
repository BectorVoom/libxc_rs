//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2001/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2001<F: Float>(t26179: F, t29544: F, t29548: F, t29554: F, t7349: F, t28640: F, t7709: F, t29562: F, t95319: F, t101899: F, t101901: F, t101903: F, t101906: F, t101907: F, t101929: F, t95314: F) -> F {
    let t110014 = t26179 * t29544;
    let t110016 = t26179 * t29548;
    let t110018 = t29554 * t7349;
    let t110020 = t7709 * t28640;
    let t110022 = t95319 * t29562;
    let t110027 = F::new(80.0) / F::new(9.0) * t110014 + F::new(40.0) / F::new(9.0) * t110016 + F::new(16.0) / F::new(9.0) * t110018 + F::new(32.0) / F::new(9.0) * t110020 - F::new(80.0) / F::new(3.0) * t110022 - t101899 - t101901 - t101903 - t101906 + F::new(176.0) / F::new(27.0) * t101907 - F::new(176.0) / F::new(27.0) * t95314 + F::new(176.0) / F::new(27.0) * t101929;
    t110027
}
