//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 185/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk185<F: Float>(t403: F, t406: F, t408: F, t413: F, t90: F, t257: F, t260: F, t266: F, t657: F, t667: F, t670: F) -> (F, F) {
    let t677 = F::new(0.77371026992393176896e-2) * t90 - F::new(0.2499945e-2) * t403 + F::new(0.604634375e-3) * t406 - F::new(0.20417003743104289064e-4) * t408 + F::new(0.20205871875e-5) * t413;
    let t679 = -F::new(0.10636476373080147432e-2) * t90 * t257 - F::new(0.21272952746160294864e-2) * t657 * t667 - t670 * t266 - t260 * t677;
    (t677, t679)
}
