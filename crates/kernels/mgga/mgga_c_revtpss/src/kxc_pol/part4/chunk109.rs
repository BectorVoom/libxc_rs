//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 109/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk109<F: Float>(t275: F, t291: F, t153: F, t159: F, t162: F, zeta_threshold: F) -> (F, F) {
    let t293 = F::new(0.621814e-1) * t275 * t291;
    let t294 = F::new(2.0) <= zeta_threshold;
    let t296 = piecewise3::<f64>(t294, t153, F::new(2.0) * t159);
    let t297 = F::new(0.0) <= zeta_threshold;
    let t298 = piecewise3::<f64>(t297, t153, F::new(0.0));
    let t300 = (t296 + t298 - F::new(2.0)) * t162;
    (t293, t300)
}
