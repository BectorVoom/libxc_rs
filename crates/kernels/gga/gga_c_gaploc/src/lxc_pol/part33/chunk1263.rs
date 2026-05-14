//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1263/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1263<F: Float>(t12215: F, t1457: F, t2103: F, t28633: F, t28636: F, t28645: F, t28675: F, t28678: F, t28681: F, t28683: F, t33412: F, t33416: F, t33419: F, t33421: F, t33429: F, t33453: F, t39091: F, t39095: F, t5771: F) -> (F,) {
    let t39136 = t33412 + 0.71500979903700853338e0 * t2103 * t1457 * t39091 + 0.14300195980740170668e1 * t5771 * t12215 + 0.14300195980740170668e1 * t2103 * t1457 * t39095 - t28633 + t28636 + t33416 + t33419 + t33421 + t28645 + t33429 + t28675 + t28678 + t28681 - 0.53964118009221795842e0 * t28683 - t33453;
    (t39136,)
}
