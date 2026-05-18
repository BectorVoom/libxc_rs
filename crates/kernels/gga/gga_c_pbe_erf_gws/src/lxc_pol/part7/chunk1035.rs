//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1035/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1035<F: Float>(t50: F, t262: F, t1412: F, t1413: F, t1416: F, t16973: F, t16978: F, t16986: F, t4373: F, t4767: F, t4770: F, t52: F, t18683: F, t59: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t18684 = F::new(1.0) / t262;
    let t18697 = piecewise3::<f64>(t51, F::new(0.0), F::new(40.0) / F::new(81.0) * t18684 * t16973 - F::new(16.0) / F::new(9.0) * t4767 * t1413 * t1416 + F::new(4.0) / F::new(3.0) * t1412 * t16986 + F::new(16.0) / F::new(9.0) * t4770 * t4373 + F::new(4.0) / F::new(3.0) * t52 * t16978);
    let t18699 = (t18683 + t18697) * t59;
    t18699
}
