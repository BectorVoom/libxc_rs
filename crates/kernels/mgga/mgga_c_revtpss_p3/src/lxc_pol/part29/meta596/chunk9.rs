//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2012/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2012<F: Float>(t98983: F, t98991: F, t99000: F, t99006: F, t92991: F, t95671: F, t98985: F, t98989: F, t98993: F, t98995: F, t98997: F, t99002: F) -> F {
    let t103273 = F::cast_from(0.4065600224742826258e-3_f64) * t98983;
    let t103276 = F::cast_from(0.80031500487063509014e-2_f64) * t98991;
    let t103280 = F::cast_from(0.22866142996303859718e-3_f64) * t99000;
    let t103283 = F::cast_from(0.57165357490759649296e-4_f64) * t99006;
    let t103284 = t103273 + F::cast_from(0.68598428988911579156e-2_f64) * t98985 - F::cast_from(0.51448821741683684367e-2_f64) * t98989 + t103276 + F::cast_from(0.34299214494455789578e-2_f64) * t98993 - t98995 / F::new(24.0) + F::cast_from(0.17149607247227894789e-1_f64) * t98997 - t103280 + F::cast_from(0.54208002996571016773e-3_f64) * t99002 - t95671 + F::cast_from(0.81312004494856525159e-4_f64) * t92991 + t103283;
    t103284
}
