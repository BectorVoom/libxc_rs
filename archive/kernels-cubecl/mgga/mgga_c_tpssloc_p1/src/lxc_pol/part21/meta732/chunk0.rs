//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2589/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2589<F: Float>(t44620: F, t461: F, t60: F, t15394: F, t1714: F, t3439: F, t3447: F, t4724: F, t697: F, t11590: F, t15376: F, t11554: F, t1706: F) -> (F, F, F, F, F) {
    let t52096 = t60 * t44620 * t461;
    let t52100 = t15394 * t1714;
    let t52109 = t3447 * t697 * t3439 * t461 * t4724;
    let t52122 = t15376 * t11590;
    let t52124 = t1706 * t11554;
    (t52096, t52100, t52109, t52122, t52124)
}
