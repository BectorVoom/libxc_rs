//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 952/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk952<F: Float>(t8972: F, t8975: F, t8908: F, t8967: F, t8969: F, t8979: F, t8983: F, t8987: F, t8990: F, t8992: F, t8995: F, t8997: F, t9097: F, t809: F, t787: F, t3330: F, t786: F) -> (F, F, F, F, F, F) {
    let t9101 = 0.32862666666666666666e0 * t8972;
    let t9102 = 0.32862666666666666666e0 * t8975;
    let t9110 = 0.39862222222222222223e0 * t8908 + 0.1898925e1 * t8967 + 0.3071625e0 * t8969 - t9101 - t9102 + 0.24647e0 * t8979 + 0.49294e0 * t8983 + 0.24647e0 * t8987 - 0.1898925e1 * t8990 - 0.9494625e0 * t8992 + 0.3071625e0 * t8995 + 0.15358125e0 * t8997;
    let t9111 = t9097 + t9110;
    let t9112 = t9111 * t809;
    let t9114 = 1.0 * t787 * t9112;
    let t9115 = t3330 * t786;
    (t9101, t9102, t9111, t9112, t9114, t9115)
}
