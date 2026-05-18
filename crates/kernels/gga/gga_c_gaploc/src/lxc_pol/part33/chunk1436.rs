//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1436/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1436<F: Float>(t12161: F, t12241: F, t1890: F, t1966: F, t28792: F, t28795: F, t28800: F, t28810: F, t33659: F, t33662: F, t33666: F, t33668: F, t33671: F, t33673: F, t33675: F, t33683: F, t33685: F, t5577: F, t590: F) -> F {
    let t39261 = t33659 - t33662 + t33666 + t33668 + t33671 - t33673 - t33675 + t33683 - t33685 + F::new(0.10224780254378866581e1) * t28792 + F::new(0.10224780254378866581e1) * t28795 + t28800 - F::new(0.1022478025437886658e1) * t5577 * t12241 - F::new(0.1022478025437886658e1) * t1966 * t1890 * t12161 * t590 - t28810;
    t39261
}
