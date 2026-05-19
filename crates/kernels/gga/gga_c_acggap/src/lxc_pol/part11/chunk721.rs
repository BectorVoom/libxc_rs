//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 721/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk721<F: Float>(t425: F, t7614: F, t431: F, t7546: F, t7550: F, t7551: F, t7558: F, t7562: F, t7567: F, t7572: F, t7574: F, t7578: F, t7581: F, t7590: F, t7593: F, t7597: F, t7602: F, t7603: F, t7607: F, t7608: F, t7612: F) -> (F, F, F) {
    let t7615 = t7614 * t425;
    let t7616 = F::cast_from(0.16006300097412701803e-1_f64) * t7615;
    let t7617 = t7614 * t431;
    let t7619 = t7546 + t7550 - F::cast_from(0.94344276868812456204e-2_f64) * t7551 - t7558 + F::cast_from(0.34299214494455789578e-2_f64) * t7562 + F::cast_from(0.18868855373762491241e-2_f64) * t7567 + t7572 + t7574 + F::cast_from(0.21437009059034868486e-2_f64) * t7578 - F::cast_from(0.21437009059034868486e-3_f64) * t7581 - t7590 - t7593 / F::new(384.0) - F::new(0.38203125e-2) * t7597 - t7602 + F::cast_from(0.85748036236139473944e-3_f64) * t7603 - t7607 - F::cast_from(0.85748036236139473944e-3_f64) * t7608 + t7612 + t7616 - F::cast_from(0.80031500487063509015e-2_f64) * t7617;
    (t7615, t7617, t7619)
}
