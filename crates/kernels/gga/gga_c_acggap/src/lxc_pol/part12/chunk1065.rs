//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1065/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1065<F: Float>(t36194: F, t36198: F, t36205: F, t36207: F, t36210: F, t36214: F, t36231: F, t31793: F, t31797: F, t31808: F, t32942: F, t36186: F, t36190: F, t36202: F, t36217: F, t36220: F, t36225: F, t36227: F) -> (F,) {
    let t37904 = 0.62896184579208304136e-2 * t36194;
    let t37905 = 0.94344276868812456204e-2 * t36198;
    let t37907 = 0.36675e0 * t36205;
    let t37908 = 0.183375e0 * t36207;
    let t37909 = 0.183375e0 * t36210;
    let t37910 = 0.183375e0 * t36214;
    let t37918 = 0.90702367218671976884e-1 * t36231;
    let t37919 = -0.37737710747524982482e-1 * t36186 + 0.56606566121287473722e-1 * t36190 - t37904 + t37905 - 0.94344276868812456207e-3 * t36202 - t37907 - t37908 - t37909 - t37910 + t36217 / 48.0 + 0.21437009059034868486e-2 * t31793 - t36220 / 6.0 + 0.183375e0 * t36225 - t36227 / 4.0 - 0.62896184579208304138e-3 * t31797 - t32942 - 0.1528125e-1 * t31808 - t37918;
    (t37919,)
}
