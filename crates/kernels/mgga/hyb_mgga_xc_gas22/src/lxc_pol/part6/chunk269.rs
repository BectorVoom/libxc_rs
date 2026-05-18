//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 269/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk269<F: Float>(t883: F, t319: F, t324: F, t313: F, t314: F, t312: F, t645: F, t99: F, t298: F, t321: F, t322: F, rho0: F, sigma0: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t884 = F::new(1.0) / t883;
    let t885 = t884 * tau0;
    let t889 = t319 * rho0;
    let t890 = F::new(1.0) / t889;
    let t891 = t890 * t324;
    let t894 = t313 * sigma0;
    let t895 = t314 * t894;
    let t896 = t312 * t895;
    let t897 = t319 * t645;
    let t899 = F::new(1.0) / t99 / t897;
    let t900 = t321 * t298;
    let t902 = F::new(1.0) / t322 / t900;
    (t884, t885, t890, t891, t894, t895, t896, t899, t900, t902)
}
