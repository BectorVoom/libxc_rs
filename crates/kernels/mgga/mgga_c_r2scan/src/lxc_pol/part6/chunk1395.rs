//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1395/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1395<F: Float>(t5207: F, t7824: F, t5957: F, t21237: F, t21240: F, t21244: F, t21247: F, t21251: F, t21254: F, t21257: F, t21262: F, t21264: F, t21268: F, t21270: F, t21272: F, t5216: F) -> (F, F) {
    let t26436 = t7824 * t5207;
    let t26438 = t7824 * t5957;
    let t26440 = t21237 + 0.84681398666666666666e-3 * t21240 - t21244 + t21247 - 0.28518989949414381017e2 * t26436 + 0.32530743900905219526e-1 * t26438 + t21251 - t21254 - t21257 - t21262 - t21264 - t21268 + t21270 + t21272;
    let t26442 = t7824 * t5216;
    (t26440, t26442)
}
