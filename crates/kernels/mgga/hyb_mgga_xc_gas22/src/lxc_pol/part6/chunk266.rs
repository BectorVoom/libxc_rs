//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 266/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk266<F: Float>(t7: F, t220: F, t291: F, t771: F, t861: F, t295: F, t313: F, t321: F, t303: F, t120: F, t306: F, t122: F, t309: F, t319: F, t324: F, t314: F, t312: F, dens_threshold: F, rho0: F, sigma0: F, tau0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t865 = piecewise3(t9, 0.0, t220 * t861 / 2.0 + t771 * t291 / 2.0);
    let t870 = t295 * t313;
    let t871 = 1.0 / t321;
    let t875 = t303 * tau0;
    let t880 = t306 * t120;
    let t883 = t309 * t122;
    let t884 = 1.0 / t883;
    let t885 = t884 * tau0;
    let t889 = t319 * rho0;
    let t890 = 1.0 / t889;
    let t891 = t890 * t324;
    let t894 = t313 * sigma0;
    let t895 = t314 * t894;
    let t896 = t312 * t895;
    (t865, t870, t871, t875, t880, t884, t885, t890, t891, t894, t895, t896)
}
