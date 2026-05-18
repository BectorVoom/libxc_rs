//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1150/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1150<F: Float>(t139390: F, t139410: F, t139413: F, t139416: F, t148545: F, t148551: F, t148555: F, t148559: F, t148563: F, t148567: F, t148571: F, t148573: F, t148578: F, t148580: F, t148583: F, t148587: F) -> F {
    let t148589 = -t139390 / F::new(54.0) - F::new(2.0) / F::new(9.0) * t148545 - F::new(2.0) / F::new(9.0) * t139410 + F::new(2.0) / F::new(3.0) * t139413 - F::new(4.0) / F::new(9.0) * t139416 - F::new(4.0) / F::new(9.0) * t148551 - F::new(4.0) * t148555 - t148559 / F::new(6.0) + t148563 / F::new(9.0) - F::new(4.0) / F::new(9.0) * t148567 + t148571 / F::new(2.0) - t148573 / F::new(27.0) + t148578 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t148580 - t148583 / F::new(36.0) - t148587 / F::new(6.0);
    t148589
}
