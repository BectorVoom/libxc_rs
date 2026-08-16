//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2559/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2559<F: Float>(t11310: F, t300: F, t15225: F, t51811: F, t51725: F, t51399: F, t51401: F, t51404: F, t51437: F, t51439: F, t51806: F, t51809: F, t51814: F, t51818: F) -> (F, F, F) {
    let t51819 = t300 * t11310;
    let t51822 = F::cast_from(0.30762056574649219974e4_f64) * t51819 * t15225 * t51811;
    let t51824 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t51725;
    let t51825 = -t51806 - t51809 + t51399 + t51401 + t51404 - t51814 + t51818 - t51822 + t51824 - t51437 - t51439;
    (t51822, t51824, t51825)
}
