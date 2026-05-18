//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1209/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1209<F: Float>(t2537: F, t7607: F, t2539: F, t2165: F, t26411: F, t26422: F, t26556: F, t2771: F, t2789: F, t36436: F, t36513: F, t7660: F, t7669: F, t899: F, t9007: F, t9018: F, t9021: F, t906: F, t91885: F, t91895: F, t91901: F, t92134: F, t92149: F) -> (F, F) {
    let t92155 = t7607 * t2537;
    let t92157 = F::new(6.0) * t92155 * t2539;
    let t92158 = F::new(6.0) * t36436 * t7660 - F::new(3.0) * t9007 * t7669 + t91885 - F::new(3.0) * t26422 * t2789 + F::new(24.0) * t36513 * t2165 * t9018 - t91895 + t91901 + F::new(6.0) * t26411 * t9021 - t899 * (t92134 + t92149) + F::new(6.0) * t2771 * t26556 * t906 - t92157;
    (t92157, t92158)
}
