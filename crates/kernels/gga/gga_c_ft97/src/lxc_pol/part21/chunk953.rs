//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 953/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk953<F: Float>(t1901: F, t26295: F, t26301: F, t26303: F, t26361: F, t28: F, t29904: F, t29907: F, t29911: F, t29915: F, t29919: F, t29923: F, t29926: F, t29932: F, t29936: F, t29940: F, t29944: F, t29948: F, t29951: F, t446: F, t89: F) -> (F,) {
    let t29955 = 2.0 / 9.0 * t26295 + 2.0 / 9.0 * t1901 * t29904 + 2.0 / 9.0 * t1901 * t29907 + 2.0 / 9.0 * t1901 * t29911 + t1901 * t29915 / 9.0 + 2.0 / 27.0 * t1901 * t29919 - 4.0 / 3.0 * t1901 * t29923 - t446 * t29926 / 3.0 - 2.0 / 9.0 * t26301 - 4.0 / 9.0 * t26303 - 2.0 / 9.0 * t1901 * t29932 + t89 * t28 * t29936 / 3.0 - 4.0 / 9.0 * t1901 * t29940 - 2.0 / 9.0 * t1901 * t29944 + 2.0 / 9.0 * t1901 * t29948 + 2.0 / 9.0 * t1901 * t29951 - 2.0 / 27.0 * t26361;
    (t29955,)
}
