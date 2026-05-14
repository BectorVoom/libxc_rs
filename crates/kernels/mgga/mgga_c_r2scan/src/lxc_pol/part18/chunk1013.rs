//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1013/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1013<F: Float>(t10918: F, t3275: F, t9573: F, t11502: F, t40681: F, t11475: F, t11479: F, t3262: F, t3579: F, t40590: F, t10610: F, t11509: F, t12574: F, t792: F, t37299: F, t12602: F, t833: F) -> (F, F, F, F, F, F, F) {
    let t42460 = t3275 * t10918 * t9573 / 2.0;
    let t42462 = 3.0 / 2.0 * t40681 * t11502;
    let t42465 = 3.0 / 2.0 * t3262 * t11479 * t11475;
    let t42467 = 5.0 / 8.0 * t3579 * t40590;
    let t42471 = 3.0 * t10610 * t11479 * t11509;
    let t42472 = t12574 * t792;
    let t42475 = 585.0 / 256.0 * t3275 * t37299 * t42472;
    let t42478 = t12602 * t833;
    (t42460, t42462, t42465, t42467, t42471, t42475, t42478)
}
