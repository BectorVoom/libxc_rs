//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 893/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk893<F: Float>(t185: F, t7751: F, t1033: F, t1795: F, t2730: F, t2753: F, t1639: F, t649: F, t1642: F, t7506: F, t7115: F, t4908: F, t616: F) -> (F, F, F, F, F) {
    let t7753 = F::new(8.0) / F::new(45.0) * t185 * t7751;
    let t7755 = F::new(4.0) / F::new(15.0) * t1033 * t1795;
    let t7757 = F::new(16.0) / F::new(45.0) * t2730 * t2753;
    let t7758 = t1639 * t649;
    let t7759 = t7758 * t1642;
    let t7760 = t7759 * t7506;
    let t7762 = F::new(8.0) / F::new(27.0) * t7115 * t7760;
    let t7764 = F::new(4.0) / F::new(15.0) * t616 * t4908;
    (t7753, t7755, t7757, t7762, t7764)
}
