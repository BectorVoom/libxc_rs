//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 779/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk779<F: Float>(t376: F, t7266: F, t89: F, t1882: F, t7231: F, t28: F, t32542: F, t32547: F, t32551: F, t32555: F, t32559: F, t32564: F, t32568: F, t32573: F, t32577: F, t32581: F, t446: F) -> (F, F, F) {
    let t32587 = t89 * t376 * t7266 / F::new(9.0);
    let t32589 = F::new(2.0) / F::new(9.0) * t1882 * t7231;
    let t32590 = F::new(4.0) / F::new(3.0) * t446 * t32542 - t446 * t32547 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t32551 + F::new(2.0) / F::new(3.0) * t446 * t32555 - F::new(2.0) * t446 * t32559 - F::new(2.0) / F::new(3.0) * t446 * t32564 + F::new(4.0) / F::new(3.0) * t446 * t32568 + F::new(2.0) / F::new(3.0) * t446 * t32573 - t446 * t32577 / F::new(9.0) + t89 * t28 * t32581 / F::new(3.0) - t32587 - t32589;
    (t32587, t32589, t32590)
}
