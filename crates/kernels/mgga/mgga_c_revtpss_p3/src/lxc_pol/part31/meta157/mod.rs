//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk793;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk794;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk795;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk796;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta157<F: Float>(t1448: F, t1450: F, t565: F, t2219: F, t2223: F, t2226: F, t2230: F, t2233: F, t2239: F, t1466: F, t602: F, t1497: F, t644: F, t1469: F, t606: F, t30: F, t33: F, t70: F, t2255: F, zeta_threshold: F, t36: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4140, t4146, t4147) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk793::<F>(t1448, t1450, t565);
        let (t4171, t4173) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk794::<F>(t2219, t2223, t2226, t2230, t2233, t2239, t1466, t602);
        let (t4178, t4181) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk795::<F>(t1497, t644, t1469, t606);
        let (t4182, t4186) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk796::<F>(t30, t33, t4181, t70, t2255, zeta_threshold);
        let t4187 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk797::<F>(t36, t4186);
    (t4140, t4146, t4147, t4171, t4173, t4178, t4181, t4182, t4186, t4187)
}
