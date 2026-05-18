//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 823/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk823<F: Float>(t580: F, t6012: F, t1890: F, t1900: F, t1905: F, t1909: F, t573: F, t577: F, t17: F, t1896: F, t576: F) -> (F, F, F, F, F, F, F) {
    let t6013 = t6012 * t580;
    let t6015 = t1890 * t1900;
    let t6017 = t1890 * t1905;
    let t6019 = t1890 * t1909;
    let t6022 = F::new(1.0) / t573 / t577;
    let t6023 = t17 * t6022;
    let t6025 = F::new(1.0) / t1896 / t576;
    (t6013, t6015, t6017, t6019, t6022, t6023, t6025)
}
