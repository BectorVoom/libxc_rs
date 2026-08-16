//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1014/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1014<F: Float>(t11837: F, t7281: F, t1871: F, t22952: F, t25883: F, t32115: F, t3266: F, t5674: F, t8411: F, t136159: F, t136189: F, t137245: F, t26006: F) -> (F, F, F, F) {
    let t144765 = t11837 * t7281;
    let t144769 = t22952 * t1871 * t32115 * t25883;
    let t144773 = t5674 * t8411 * t32115 * t3266;
    let t144777 = t136159 * t137245 * t136189 * t26006;
    (t144765, t144769, t144773, t144777)
}
