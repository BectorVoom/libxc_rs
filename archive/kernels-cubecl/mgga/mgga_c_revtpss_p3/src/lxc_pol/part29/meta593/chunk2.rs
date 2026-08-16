//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1979/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1979<F: Float>(t98185: F, t98187: F, t98193: F, t98200: F, t98202: F, t98206: F, t94468: F, t96321: F, t96322: F, t98189: F, t98191: F, t98197: F, t98204: F) -> F {
    let t102508 = F::cast_from(0.4065600224742826258e-3_f64) * t98185;
    let t102509 = F::cast_from(0.10164000561857065645e-3_f64) * t98187;
    let t102512 = F::cast_from(0.32012600194825403606e-1_f64) * t98193;
    let t102515 = F::cast_from(0.40656002247428262579e-4_f64) * t98200;
    let t102516 = F::cast_from(0.4065600224742826258e-3_f64) * t98202;
    let t102518 = F::cast_from(0.2032800112371413129e-2_f64) * t98206;
    let t102519 = t102508 - t102509 - F::cast_from(0.68598428988911579156e-2_f64) * t98189 + F::cast_from(0.34299214494455789578e-2_f64) * t98191 - t102512 - F::cast_from(0.50820002809285328225e-4_f64) * t94468 - t98197 / F::cast_from(2.0_f64) - t96321 + t102515 + t96322 + t102516 + F::cast_from(0.34299214494455789578e-1_f64) * t98204 + t102518;
    t102519
}
