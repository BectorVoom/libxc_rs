//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1135/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1135<F: Float>(t30: F, t265: F, t393: F, t1544: F, t2071: F, t207: F, t8019: F, t1583: F, t1940: F, t198: F, t2403: F, t7432: F, t892: F, t1468: F, t1469: F, t2078: F, t45: F, t7787: F, t7991: F, t8020: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8031 = t2071 * t1544;
    let t8034 = t207 * t8019;
    let t8039 = -t1583 * t1940 * t7432 + t198 * t8034 * t892 + F::new(3.0) * t2403 * t8031;
    let t8040 = piecewise3::<F>(t394, F::new(0.0), t8039);
    let t8045 = piecewise3::<F>(t120, F::new(3.0) / F::new(2.0) * t2403 * t7991 + t1940 * t8020 * t30 / F::new(2.0) - t1940 * t7432 * t7787 / F::new(2.0) + t1940 * t2071 * t1468 / F::new(2.0), t2078 * t1469 / F::new(2.0) + t8040 * t45 / F::new(2.0));
    (t8031, t8039, t8040, t8045)
}
