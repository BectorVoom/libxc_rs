//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1395/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1395<F: Float>(t4083: F, t9955: F, t12198: F, t1105: F, t353: F, t4228: F, t4386: F, t54952: F, t55796: F, t55807: F, t55809: F, t57488: F, t57495: F, t57497: F, t57500: F, t57506: F, t57509: F, t57514: F, t57516: F, t57542: F, t6793: F, t8793: F) -> F {
    let t58821 = t9955 * t4083;
    let t58823 = t12198 * t4083;
    let t58835 = t4386 * t353 * t4228 * t1105;
    let t58839 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t57488 + t57495 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t58821 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t58823 - t57497 / F::cast_from(48.0_f64) - t57500 / F::cast_from(96.0_f64) - t57506 / F::cast_from(24.0_f64) - t57509 / F::cast_from(48.0_f64) + t57514 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t57516 + t8793 * t54952 / F::cast_from(24.0_f64) + t6793 * t58835 / F::cast_from(24.0_f64) + t55796 - t55807 - t55809 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t57542;
    t58839
}
