//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 833/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk833<F: Float>(t7335: F, t8355: F, t7345: F, t1001: F, t1475: F, t1970: F, t236: F, t9210: F, t35455: F, t8451: F, t7421: F, t8571: F) -> (F, F, F, F, F) {
    let t38608 = t7335 * t8355;
    let t38610 = t7345 * t8355;
    let t38615 = t1970 * t9210 * t236 * t1475 * t1001;
    let t38617 = t8451 * t35455;
    let t38619 = t8571 * t7421;
    (t38608, t38610, t38615, t38617, t38619)
}
