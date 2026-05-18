//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 792/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk792<F: Float>(t11054: F, t954: F, t4927: F, t639: F, t7845: F, t11020: F, t11023: F, t12323: F, t225: F, t11026: F, t11038: F, t12497: F, t1714: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12821 = t11054 * t954;
    let t12822 = t4927 * t12821;
    let t12824 = F::new(8.0) / F::new(15.0) * t639 * t12822;
    let t12825 = F::new(4.0) / F::new(45.0) * t7845;
    let t12827 = F::new(32.0) / F::new(45.0) * t11020;
    let t12828 = F::new(16.0) / F::new(45.0) * t11023;
    let t12829 = t12323 * t225;
    let t12832 = F::new(4.0) / F::new(15.0) * t11026;
    let t12834 = F::new(8.0) / F::new(45.0) * t11038;
    let t12837 = t1714 * t12497;
    (t12821, t12822, t12824, t12825, t12827, t12828, t12829, t12832, t12834, t12837)
}
