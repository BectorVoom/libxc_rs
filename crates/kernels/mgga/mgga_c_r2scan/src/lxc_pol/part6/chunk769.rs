//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 769/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk769<F: Float>(t5137: F, t552: F, t551: F, t1632: F, t2185: F, t2184: F, t122: F, t2161: F, t625: F) -> (F, F, F, F, F) {
    let t5138 = t552 * t5137;
    let t5139 = t551 * t5138;
    let t5142 = t1632 * t2185;
    let t5143 = t551 * t5142;
    let t5144 = t2184 * t5143;
    let t5146 = t2161 * t122;
    let t5147 = t625 * t5146;
    (t5139, t5143, t5144, t5146, t5147)
}
