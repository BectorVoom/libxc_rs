//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1982/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1982<F: Float>(t98258: F, t98260: F, t98269: F, t94514: F, t94520: F, t94527: F, t94530: F, t94534: F, t94537: F, t94540: F, t96341: F, t96342: F) -> F {
    let t102548 = F::cast_from(0.11433071498151929859e-3_f64) * t98258;
    let t102549 = F::new(35.0) / F::new(108.0) * t98260;
    let t102557 = F::new(7.0) / F::new(36.0) * t98269;
    let t102558 = -t102548 - t102549 - F::new(7.0) / F::new(24.0) * t94514 - F::new(35.0) / F::new(54.0) * t94520 - t96341 + t96342 - F::cast_from(0.24390552529390783699e-2_f64) * t94527 + F::cast_from(0.11433071498151929859e-3_f64) * t94530 - F::cast_from(0.57165357490759649295e-3_f64) * t94534 + F::cast_from(0.2032800112371413129e-4_f64) * t94537 - F::cast_from(0.14457274399185490174e-3_f64) * t94540 + t102557;
    t102558
}
