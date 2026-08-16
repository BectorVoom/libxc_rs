//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1142/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1142<F: Float>(t142636: F, t142647: F, t142662: F, t1466: F, t193: F, t28863: F, t28868: F, t33983: F, t34278: F, t36013: F, t36017: F, t36093: F, t36103: F, t6210: F, t6261: F, t6263: F, t6267: F, t6963: F, t7129: F, t7581: F, t880: F) -> F {
    let t153548 = t6210 * t36017 / F::cast_from(6.0_f64) + t142636 / F::cast_from(9.0_f64) - t1466 * t193 * t33983 * t28868 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6210 * t36013 + t7581 * t28863 / F::cast_from(6.0_f64) - t6963 * t34278 / F::cast_from(3.0_f64) + t1466 * t193 * t36103 * t880 / F::cast_from(6.0_f64) - t142647 / F::cast_from(9.0_f64) + t36093 * t6267 / F::cast_from(6.0_f64) + t36093 * t6263 / F::cast_from(6.0_f64) + t1466 * t193 * t6261 * t7129 / F::cast_from(3.0_f64) - t142662;
    t153548
}
