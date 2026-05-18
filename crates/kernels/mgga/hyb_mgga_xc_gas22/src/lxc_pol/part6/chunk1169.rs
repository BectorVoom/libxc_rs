//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1169/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1169<F: Float>(t6578: F, t783: F, t20624: F, t20688: F, t2186: F, t2232: F, t230: F, t2306: F, t6669: F, t2243: F, t2250: F, t6682: F, t816: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20846 = t783 * t6578;
    let t20853 = F::new(0.31003950617283950618e1) * t20624;
    let t20867 = F::new(0.13388493827160493828e1) * t20688;
    let t20895 = t230 / t2232 / t2186;
    let t20904 = F::new(0.96141975308641975307e-1) * t20624;
    let t20934 = t2306 * t6669;
    let t20960 = F::new(0.17757530864197530864e0) * t20624;
    let t20972 = t2243 * t2250;
    let t20975 = t816 * t6682;
    (t20846, t20853, t20867, t20895, t20904, t20934, t20960, t20972, t20975)
}
