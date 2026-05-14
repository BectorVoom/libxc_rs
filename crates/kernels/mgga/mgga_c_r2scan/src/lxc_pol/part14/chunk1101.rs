//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1101/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1101<F: Float>(t322: F, t41940: F, t41971: F, t42003: F, t42035: F, t42067: F, t42098: F, t42131: F, t12029: F, t37271: F, t12094: F, t37282: F, t12215: F, t40549: F, t10687: F, t12056: F, t3275: F) -> (F, F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t42133 = piecewise5(t323, t41940, t331, t41971 + t42003 + t42035 + t42067, t42098 + t42131);
    let t42136 = 5.0 / 8.0 * t37271 * t12029;
    let t42138 = 15.0 / 8.0 * t37282 * t12094;
    let t42140 = 3.0 * t40549 * t12215;
    let t42143 = t3275 * t12056 * t10687 / 4.0;
    (t42133, t42136, t42138, t42140, t42143)
}
