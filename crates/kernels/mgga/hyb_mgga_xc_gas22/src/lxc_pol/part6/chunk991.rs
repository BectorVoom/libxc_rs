//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 991/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk991<F: Float>(t1006: F, t9195: F, t3557: F, t997: F, t1007: F, t2594: F, t3560: F, t8988: F, t8992: F, t8995: F, t8999: F, t9101: F, t9103: F, t9106: F, t9108: F, t9110: F, t9170: F, t998: F) -> (F, F, F) {
    let t9196 = t9195 * t1006;
    let t9199 = t3557 * t997;
    let t9204 = t8988 - t8992 - t8995 - t8999 - t9101 - t9103 - t9106 - t9108 - t9110 - t9170 + F::new(0.5848223622634646207e0) * t998 * t9196 + F::new(0.11696447245269292414e1) * t9199 * t1007 + F::new(0.5848223622634646207e0) * t3560 * t2594;
    (t9196, t9199, t9204)
}
