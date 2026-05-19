//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 491/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk491<F: Float>(t3918: F, t473: F, t3944: F, t1341: F, t187: F, t4114: F, t1588: F, t1591: F) -> (F, F, F, F, F) {
    let t4366 = t473 * t3918;
    let t4373 = t473 * t3944;
    let t4381 = t187 * t1341;
    let t4399 = F::cast_from(0.38691203703703703703e-3_f64) * t4114;
    let t4409 = t1588 * t1591;
    (t4366, t4373, t4381, t4399, t4409)
}
