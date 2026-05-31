//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 114/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk114<F: Float>(t273: F, t276: F, t279: F, t285: F) -> (F, F, F, F) {
    let t307 = F::cast_from(0.705945e1_f64) * t276 + F::cast_from(0.1549425e1_f64) * t273 + F::cast_from(0.420775e0_f64) * t279 + F::cast_from(0.1562925e0_f64) * t285;
    let t310 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t307;
    let t311 = F::ln(t310);
    let t315 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t273;
    (t307, t310, t311, t315)
}
