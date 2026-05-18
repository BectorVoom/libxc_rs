//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1136/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1136<F: Float>(t143040: F, t143112: F, t28776: F, t33822: F, t33821: F, t3628: F, t3746: F, t6307: F, t143041: F, t143042: F, t28816: F, t143274: F, t143276: F, t143321: F, t143324: F, t143327: F, t153388: F, t153390: F, t153395: F, t153399: F, t153402: F, t153405: F, t153414: F, t153418: F) -> (F, F, F, F) {
    let t153422 = t143040 * t143112 * t33822 * t28776;
    let t153427 = t6307 * t3628 * t33821 * t33822 * t3746;
    let t153431 = t143040 * t143041 * t143042 * t28816;
    let t153432 = -t143274 - t153388 / F::new(27.0) - F::new(4.0) / F::new(27.0) * t153390 + t153395 / F::new(18.0) - t153399 / F::new(3.0) + t153402 / F::new(9.0) - t153405 / F::new(9.0) - t143276 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t143321 - F::new(4.0) / F::new(9.0) * t143324 - F::new(2.0) / F::new(9.0) * t143327 - F::new(20.0) / F::new(3.0) * t153414 + F::new(8.0) / F::new(3.0) * t153418 - F::new(2.0) * t153422 - F::new(4.0) / F::new(9.0) * t153427 + t153431;
    (t153422, t153427, t153431, t153432)
}
