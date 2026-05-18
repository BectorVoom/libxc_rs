//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1002/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1002<F: Float>(t10860: F, t7630: F, t8521: F, t959: F, t2660: F, t8793: F, t787: F, t8792: F) -> (F, F, F, F) {
    let t10862 = F::new(0.71500979903700853338e0) * t7630 * t10860;
    let t10863 = t8521 * t959;
    let t10864 = F::new(0.14896037479937677779e-1) * t10863;
    let t10866 = F::new(0.10725146985555128001e1) * t8793 * t2660;
    let t10867 = t787 * t8792;
    (t10862, t10864, t10866, t10867)
}
