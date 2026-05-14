//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1272/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1272<F: Float>(t277: F, t9365: F, t2201: F, t2687: F, t8279: F, t2207: F, t2837: F, t8270: F, t2202: F, t9268: F, t22709: F, t6583: F, t8764: F, t20994: F, t3105: F, t6118: F, t8849: F) -> (F, F, F, F, F, F, F) {
    let t29585 = t277 * t9365;
    let t29599 = t2201 * t8279 * t2687;
    let t29604 = t2207 * t2837 * t8270;
    let t29613 = t2201 * t9268 * t2202;
    let t29635 = t6583 * t22709 * t8764;
    let t29670 = t20994 * t3105;
    let t29674 = t6118 * t8849;
    (t29585, t29599, t29604, t29613, t29635, t29670, t29674)
}
