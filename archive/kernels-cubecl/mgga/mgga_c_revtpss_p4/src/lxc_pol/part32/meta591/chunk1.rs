//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1923/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1923<F: Float>(t14485: F, t26497: F, t4481: F, t95743: F, t10073: F, t25402: F, t7056: F, t7997: F, t26519: F, t98867: F, t98937: F, t98949: F) -> (F, F, F, F, F, F) {
    let t103220 = t26497 * t14485;
    let t103224 = F::cast_from(0.19514881078765566038e-1_f64) * t95743 * t4481;
    let t103234 = t10073 * t7056 * t25402 * t7997;
    let t103240 = t98867 * t26519;
    let t103247 = F::cast_from(0.16006300097412701803e-1_f64) * t98937;
    let t103254 = F::cast_from(0.32012600194825403606e-1_f64) * t98949;
    (t103220, t103224, t103234, t103240, t103247, t103254)
}
