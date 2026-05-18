//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 947/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk947<F: Float>(t39277: F, t8831: F, t1987: F, t45561: F, t1990: F, t1979: F, t1982: F, t458: F, t9734: F, t674: F, t7715: F, t9774: F) -> (F, F, F, F, F) {
    let t45754 = t39277 * t8831;
    let t45757 = t45561 * t1987;
    let t45759 = t45561 * t1990;
    let t45763 = t9734 * t458 * t1979 * t1982;
    let t45766 = t9774 * t7715 * t674;
    (t45754, t45757, t45759, t45763, t45766)
}
