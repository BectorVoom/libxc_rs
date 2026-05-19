//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1431/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1431<F: Float>(t35215: F, t544: F, t32745: F, t488: F, t31747: F, t493: F, t4391: F, t549: F, t7893: F, t10430: F, t2487: F, t6985: F) -> (F, F, F, F) {
    let t35216 = t544 * t35215;
    let t35219 = F::cast_from(0.79445533226334281486e-1_f64) * t35216 * t32745 * t488;
    let t35220 = t493 * t31747;
    let t35225 = t4391 * t549 * t7893;
    let t35226 = F::cast_from(0.11916829983950142223e0_f64) * t35225;
    let t35228 = t2487 * t6985 * t10430;
    (t35219, t35220, t35226, t35228)
}
