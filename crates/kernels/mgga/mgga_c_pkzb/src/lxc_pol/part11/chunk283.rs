//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 283/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk283<F: Float>(t24: F, t135: F, t273: F, t830: F, t855: F, t895: F, t897: F, t902: F, t955: F, t957: F, t507: F, t422: F, t423: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t960 = t135 * t273 * t955 * t957 - t830 + t855 + t895 + t897 - t902;
    let t962 = piecewise3::<F>(t90, F::new(0.0), t507);
    let t966 = piecewise3::<F>(t332, F::new(0.0), t422 * t962 / F::new(2.0) + t960 * t423 / F::new(2.0));
    (t960, t962, t966)
}
