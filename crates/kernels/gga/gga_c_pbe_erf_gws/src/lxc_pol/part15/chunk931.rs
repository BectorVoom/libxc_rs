//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 931/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk931<F: Float>(t8197: F, t542: F, t974: F, t496: F, t127: F, t1504: F, t5810: F, t5819: F, t5836: F, t8181: F, t8182: F, t8186: F, t8187: F, t8193: F, t8194: F) -> (F, F, F) {
    let t8198 = F::cast_from(0.64956111111111111111e0_f64) * t8197;
    let t8199 = t542 * t974;
    let t8200 = t496 * t8199;
    let t8202 = -F::new(0.195872e1) * t5810 - t8181 - t8182 - t5819 / F::new(2.0) - t8186 - F::new(0.293808e2) * t127 * t8187 * t1504 - t8193 - F::new(0.146904e1) * t127 * t8194 - t8198 - F::new(2.0) / F::new(9.0) * t8200 + t5836;
    (t8198, t8199, t8202)
}
