//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1093/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1093<F: Float>(t1261: F, t17448: F, t17605: F, t17792: F, t1782: F, t21213: F, t21283: F, t21285: F, t21287: F, t24787: F, t24794: F, t24798: F, t24804: F, t24808: F, t3625: F, t5373: F, t6640: F, t6659: F, t6663: F) -> F {
    let t24815 = F::cast_from(0.42874018118069736972e-3_f64) * t21283 + F::cast_from(0.14481890564325777821e-1_f64) * t21285 - F::cast_from(0.45732285992607719436e-2_f64) * t21287 - F::new(11.0) / F::new(108.0) * t21213 * t1782 + t17792 / F::new(54.0) - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t24787 + F::cast_from(0.45732285992607719436e-2_f64) * t17605 * t6640 - F::cast_from(0.42874018118069736972e-3_f64) * t3625 * t24794 - F::cast_from(0.85748036236139473944e-3_f64) * t3625 * t24798 - F::cast_from(0.85748036236139473944e-3_f64) * t17448 * t6640 + F::cast_from(0.7145669686344956162e-3_f64) * t3625 * t24804 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t24808 + t5373 * t6659 / F::new(36.0) + t5373 * t6663 / F::new(18.0);
    t24815
}
