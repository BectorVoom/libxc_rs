//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 703/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk703<F: Float>(t32571: F, t452: F, t488: F, t379: F, t447: F, t7288: F, t103: F, t32365: F, t82: F, t376: F, t7266: F, t89: F, t1882: F, t7231: F, t28: F, t32542: F, t32547: F, t32551: F, t32555: F, t32559: F, t32564: F, t32568: F, t446: F) -> (F, F, F, F, F, F) {
    let t32573 = t452 * t488 * t32571;
    let t32577 = t447 * t7288 * t379;
    let t32581 = t82 * t32365 * t103;
    let t32587 = t89 * t376 * t7266 / 9.0;
    let t32589 = 2.0 / 9.0 * t1882 * t7231;
    let t32590 = 4.0 / 3.0 * t446 * t32542 - t446 * t32547 / 3.0 - 2.0 / 3.0 * t446 * t32551 + 2.0 / 3.0 * t446 * t32555 - 2.0 * t446 * t32559 - 2.0 / 3.0 * t446 * t32564 + 4.0 / 3.0 * t446 * t32568 + 2.0 / 3.0 * t446 * t32573 - t446 * t32577 / 9.0 + t89 * t28 * t32581 / 3.0 - t32587 - t32589;
    (t32573, t32577, t32581, t32587, t32589, t32590)
}
