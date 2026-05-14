//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1323/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1323<F: Float>(t1003: F, t11343: F, t11541: F, t23078: F, t23082: F, t2574: F, t260: F, t2605: F, t2609: F, t2613: F, t32035: F, t32047: F, t32049: F, t32060: F, t32074: F, t32085: F, t32181: F, t32202: F, t32246: F, t32299: F, t32357: F, t32391: F, t3608: F, t3618: F, t3623: F, t4359: F, t4390: F, t7434: F, t9318: F, t9602: F, t986: F, t995: F) -> (F,) {
    let t32396 = -0.69263436422725855034e2 * t9318 * t3623 - 0.91082604192152556044e5 * t1003 * t23078 * t4359 * t23082 * t2574 - 0.5848223622634646207e0 * t1003 * t986 * t32035 * t995 - 0.5848223622634646207e0 * t11541 * t2613 + 0.11696447245269292414e1 * t11541 * t2609 - 0.23392894490538584828e1 * t9318 * t3618 + t32047 - t32049 - t32060 - 0.69263436422725855036e2 * t2605 * t11343 - 0.5848223622634646207e0 * t7434 * t4390 + 0.23392894490538584828e1 * t3608 * t9602 + t260 * (t32074 + t32085 + t32181 + t32202 + t32246 + t32299 + t32357 + t32391);
    (t32396,)
}
