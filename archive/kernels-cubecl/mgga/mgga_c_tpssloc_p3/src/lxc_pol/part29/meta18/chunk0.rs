//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 132/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk132<F: Float>(t273: F, t276: F, t279: F, t285: F) -> (F, F, F) {
    let t320 = F::cast_from(0.51785e1_f64) * t276 + F::cast_from(0.905775e0_f64) * t273 + F::cast_from(0.1100325e0_f64) * t279 + F::cast_from(0.1241775e0_f64) * t285;
    let t323 = F::cast_from(1.0_f64) + F::cast_from(0.29608749977793437516e2_f64) / t320;
    let t324 = F::ln(t323);
    (t320, t323, t324)
}
