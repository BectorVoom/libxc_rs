//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1260/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1260<F: Float>(t1022: F, t7275: F, t2009: F, t2021: F, t10925: F, t5724: F, t1402: F, t2033: F, t3473: F, t25260: F, t787: F, t9824: F) -> (F, F, F, F) {
    let t33360 = t7275 * t1022;
    let t33363 = F::new(0.71500979903700853338e0) * t2021 * t33360 * t2009;
    let t33365 = F::new(0.35750489951850426669e0) * t10925 * t5724;
    let t33367 = t2033 * t1402 * t3473;
    let t33368 = F::new(0.89376224879626066674e-1) * t33367;
    let t33375 = t787 * t25260 * t9824;
    (t33363, t33365, t33368, t33375)
}
