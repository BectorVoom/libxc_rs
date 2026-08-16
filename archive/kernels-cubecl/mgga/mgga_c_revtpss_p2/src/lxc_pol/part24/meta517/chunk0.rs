//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1538/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1538<F: Float>(t1678: F, t19462: F, t1086: F, t23959: F, t23997: F, t3153: F, t3154: F, t6299: F, t12050: F, t357: F, t11631: F, t24042: F, t359: F) -> (F, F, F, F, F, F, F) {
    let t80173 = t19462 * t1678;
    let t80243 = t23959 * t1086;
    let t80264 = t23997 * t3153;
    let t80277 = t3154 * t6299;
    let t80350 = t12050 * t357;
    let t80358 = t11631 * t6299;
    let t80396 = t359 * t24042;
    (t80173, t80243, t80264, t80277, t80350, t80358, t80396)
}
