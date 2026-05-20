//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1292/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1292<F: Float>(t128903: F, t128904: F, t128906: F, t128910: F, t128917: F, t1843: F, t1911: F, t32107: F, t32109: F, t32112: F, t33286: F, t33296: F, t34399: F, t5517: F, t7489: F, t7539: F, t8463: F, t8886: F) -> F {
    let t131045 = -t1843 * t33286 + t1911 * t33296 + F::new(3.0) * t34399 * t7489 - t34399 * t7539 - t5517 * t8886 + t128903 - t128904 + t128906 - t128910 - t128917 - t32107 - t32109 - t32112 - t8463;
    t131045
}
