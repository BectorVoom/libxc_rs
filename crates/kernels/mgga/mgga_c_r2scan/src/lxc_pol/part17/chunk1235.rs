//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1235/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1235<F: Float>(t39977: F, t39982: F, t41634: F, t41635: F, t41636: F, t41637: F, t41638: F, t41642: F, t41644: F, t43447: F, t43451: F, t43454: F) -> F {
    let t44396 = F::new(0.27944763721877274748e0) * t43447 - F::new(0.46574606203128791246e-1) * t43451 + t41634 + t41635 + F::new(0.12805040077930161442e0) * t43454 + t41636 + t41637 + t41638 - F::new(0.85366933852867742947e0) * t39977 - t41642 - F::new(0.92461031893912198008e0) * t39982 + t41644;
    t44396
}
