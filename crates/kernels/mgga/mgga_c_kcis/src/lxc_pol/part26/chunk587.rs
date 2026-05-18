//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 587/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk587<F: Float>(t1506: F, t6048: F, t3795: F, t4318: F, t5469: F, t5472: F, t5475: F, t5479: F, t1563: F, t2072: F, t1571: F, t2080: F) -> (F, F, F, F) {
    let t6049 = t1506 * t6048;
    let t6072 = t4318 + F::new(0.57077777777777777777e-2) * t3795 + F::new(0.57077777777777777777e-2) * t5469 - F::new(0.11415555555555555555e-1) * t5472 + F::new(0.34246666666666666666e-1) * t5475 + F::new(0.34246666666666666666e-1) * t5479;
    let t6075 = t2072 * t1563;
    let t6080 = t2080 * t1571;
    (t6049, t6072, t6075, t6080)
}
