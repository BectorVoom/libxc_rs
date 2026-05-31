//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 268/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk268<F: Float>(t7: F, t220: F, t291: F, t771: F, t861: F, t295: F, t313: F, t321: F, t303: F, t120: F, t306: F, t122: F, t309: F, dens_threshold: F, rho0: F, tau0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t865 = piecewise3::<F>(t9, F::cast_from(0.0_f64), t220 * t861 / F::cast_from(2.0_f64) + t771 * t291 / F::cast_from(2.0_f64));
    let t870 = t295 * t313;
    let t871 = F::cast_from(1.0_f64) / t321;
    let t875 = t303 * tau0;
    let t880 = t306 * t120;
    let t883 = t309 * t122;
    (t865, t870, t871, t875, t880, t883)
}
