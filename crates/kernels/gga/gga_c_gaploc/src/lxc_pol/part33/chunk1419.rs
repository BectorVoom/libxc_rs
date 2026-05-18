//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1419/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1419<F: Float>(t28283: F, t28289: F, t28290: F, t28296: F, t28307: F, t28312: F, t33009: F, t33013: F, t33018: F, t33021: F, t33024: F, t33030: F, t33033: F, t33041: F, t33048: F, t33055: F) -> F {
    let t38998 = -t33009 - t33013 + t33018 + t33021 + t33024 - F::new(0.15337170381568299871e1) * t28283 - t28289 - F::new(0.76685851907841499354e0) * t28290 + F::new(0.15337170381568299871e1) * t28296 - F::new(0.76685851907841499354e0) * t28307 + t28312 + t33030 + t33033 + t33041 - t33048 - t33055;
    t38998
}
