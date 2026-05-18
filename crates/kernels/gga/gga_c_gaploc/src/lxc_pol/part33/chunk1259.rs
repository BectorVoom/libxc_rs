//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1259/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1259<F: Float>(t33375: F, t123: F, t8720: F, t883: F, t2684: F, t2685: F, t2464: F, t2465: F, t8469: F, t10879: F, t7416: F, t10883: F) -> (F, F, F, F, F, F) {
    let t33376 = F::new(0.14896037479937677779e-1) * t33375;
    let t33378 = t8720 * t123 * t883;
    let t33380 = t2684 * t2685 * t33378;
    let t33381 = F::new(0.19171462976960374838e0) * t33380;
    let t33384 = t2684 * t2464 * t2465 * t8469;
    let t33385 = F::new(0.85206502119823888168e-1) * t33384;
    let t33386 = t7416 * t10879;
    let t33387 = F::new(0.38342925953920749676e0) * t33386;
    let t33388 = t7416 * t10883;
    (t33376, t33378, t33381, t33385, t33387, t33388)
}
