//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 838/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk838<F: Float>(t1990: F, t45561: F, t1979: F, t1982: F, t458: F, t9734: F, t674: F, t7715: F, t9774: F, t1997: F, t1734: F, t2084: F, t2134: F, t27: F, t2286: F, t38355: F) -> (F, F, F, F, F) {
    let t45759 = t45561 * t1990;
    let t45763 = t9734 * t458 * t1979 * t1982;
    let t45766 = t9774 * t7715 * t674;
    let t45767 = t45766 * t1997;
    let t45775 = t2134 * t27 * t2084 * t1734;
    let t45777 = t38355 * t2286;
    (t45759, t45763, t45767, t45775, t45777)
}
