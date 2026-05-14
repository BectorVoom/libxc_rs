//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 974/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk974<F: Float>(t42931: F, t42934: F, t42937: F, t42940: F, t42943: F, t42948: F, t42951: F, t42954: F, t42956: F, t42961: F, t47587: F, t2558: F, t39002: F, t9647: F, t12311: F, t2554: F, t7064: F) -> (F, F, F) {
    let t47592 = -0.85450291446024714263e-3 * t47587 - 0.32043859292259267849e-3 * t42931 - t42934 - t42937 - t42940 + t42943 + t42948 - 0.96131577876777803547e-3 * t42951 - t42954 + 0.64087718584518535698e-3 * t42956 - t42961;
    let t47594 = t9647 * t39002 * t2558;
    let t47597 = t7064 * t12311 * t2554;
    (t47592, t47594, t47597)
}
