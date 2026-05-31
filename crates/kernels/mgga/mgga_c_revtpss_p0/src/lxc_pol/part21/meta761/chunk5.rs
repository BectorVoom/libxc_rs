//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2701/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2701<F: Float>(t1907: F, t47672: F, t1343: F, t1868: F, t198: F, t40079: F, t4139: F, t47152: F, t47638: F, t48328: F, t48329: F, t48330: F, t48332: F, t48334: F, t48336: F, t48421: F, t5541: F, t9590: F) -> F {
    let t49668 = t1907 * t47672;
    let t49675 = F::cast_from(3.0_f64) * t1343 * t198 * t48421 + F::cast_from(6.0_f64) * t1868 * t4139 * t47638 - F::cast_from(6.0_f64) * t49668 * t5541 * t9590 - t40079 + t47152 - t48328 - t48329 + t48330 - t48332 + t48334 + t48336;
    t49675
}
