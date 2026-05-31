//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1726/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1726<F: Float>(t25223: F, t25225: F, t25229: F, t25235: F, t25238: F, t25246: F, t25248: F, t26450: F, t26454: F, t26457: F, t26472: F) -> F {
    let t26473 = t26450 + F::cast_from(0.32012600194825403606e-1_f64) * t25223 - F::cast_from(0.34299214494455789578e-2_f64) * t25225 + F::cast_from(0.57165357490759649296e-4_f64) * t25229 - t26454 - F::cast_from(0.4065600224742826258e-3_f64) * t25235 + t25238 / F::cast_from(8.0_f64) + t26457 - F::cast_from(0.10164000561857065645e-3_f64) * t25246 + F::cast_from(0.17149607247227894789e-1_f64) * t25248 + t26472;
    t26473
}
