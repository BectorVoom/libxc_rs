//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1142/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1142(t142636: f64, t142647: f64, t142662: f64, t1466: f64, t193: f64, t28863: f64, t28868: f64, t33983: f64, t34278: f64, t36013: f64, t36017: f64, t36093: f64, t36103: f64, t6210: f64, t6261: f64, t6263: f64, t6267: f64, t6963: f64, t7129: f64, t7581: f64, t880: f64) -> f64 {
    let t153548 = t6210 * t36017 / 6.0_f64 + t142636 / 9.0_f64 - t1466 * t193 * t33983 * t28868 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t6210 * t36013 + t7581 * t28863 / 6.0_f64 - t6963 * t34278 / 3.0_f64 + t1466 * t193 * t36103 * t880 / 6.0_f64 - t142647 / 9.0_f64 + t36093 * t6267 / 6.0_f64 + t36093 * t6263 / 6.0_f64 + t1466 * t193 * t6261 * t7129 / 3.0_f64 - t142662;
    t153548
}
