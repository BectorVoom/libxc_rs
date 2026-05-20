//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2267/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2267<F: Float>(t33: F, t265: F, t502: F, t107868: F, t112989: F, t108049: F, t1469: F, t18281: F, t2159: F, t29329: F, t30936: F, t4186: F, t57: F, t5825: F, t606: F, t7677: F, t8227: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t112990 = piecewise3::<F>(t503, t112989, t107868);
    let t113002 = piecewise3::<F>(t400, t108049, t112990 * t57 / F::new(2.0) - t30936 * t606 / F::new(2.0) - t29329 * t1469 - t8227 * t4186 - t7677 * t5825 / F::new(2.0) - t2159 * t18281 / F::new(2.0));
    t113002
}
